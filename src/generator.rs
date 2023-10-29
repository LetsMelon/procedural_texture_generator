use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::prelude::DiGraph;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::{Directed, Graph};
use rusvid_core::plane::Plane;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::library::output::Output;
use crate::link::Link;
use crate::node::Node;

#[derive(Debug)]
pub struct Generator {
    pub(crate) internal_graph: Graph<Rc<RefCell<dyn Node>>, (), Directed>,

    named_links: HashMap<Link, String>,
    output_node: NodeIndex,
}

unsafe impl Sync for Generator {}
unsafe impl Send for Generator {}

impl Generator {
    pub fn new() -> Self {
        let mut g = Generator {
            internal_graph: Graph::new(),

            named_links: HashMap::new(),
            // TODO maybe use an option for this because we set this value in the following lines and '0' is a fake value
            output_node: NodeIndex::new(0),
        };

        g.output_node = g.add_node(Output::new());

        g
    }

    pub fn output_node(&self) -> NodeIndex {
        self.output_node
    }

    pub fn add_node<N: Node + 'static>(&mut self, node: N) -> NodeIndex {
        self.internal_graph.add_node(Rc::new(RefCell::new(node)))
    }

    pub fn add_edge(&mut self, link: Link) -> EdgeIndex {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        self.add_edge_named(
            link,
            format!("_{}", COUNTER.fetch_add(1, Ordering::Relaxed)),
        )
    }

    pub fn add_edge_named<S: Into<String>>(&mut self, link: Link, name: S) -> EdgeIndex {
        let name: String = name.into();
        self.named_links.insert(link.clone(), name);

        self.internal_graph
            .add_edge(link.input_node, link.output_node, ())
    }

    pub fn connected_nodes_to_output(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        let mut g = self.internal_graph.clone();
        g.reverse();

        let mut used_nodes_for_output = Vec::with_capacity(g.node_count());
        let mut dfs = Dfs::new(&g, self.output_node());
        while let Some(nx) = dfs.next(&g) {
            used_nodes_for_output.push(g[nx].clone());
        }
        used_nodes_for_output.reverse();

        used_nodes_for_output
    }

    fn nodes_as_tree(&self) -> Option<RelationsBetweenNodes> {
        // TODO is there not a better way to transform the graph into a tree like data structure, and without using `Rc`s and `RefCell` everywhere, maybe chatgpt has a suitable approach https://chat.openai.com/share/8a425597-f5ba-4106-ad9c-ede2bdac87ba
        // TODO is here really the mst needed?... maybe we can use the links from `self.named_links`
        let mst = DiGraph::<_, _>::from_elements(min_spanning_tree(&self.internal_graph));

        let mut map: HashMap<NodeIndex, Rc<RefCell<RelationsBetweenNodes>>> = HashMap::new();

        for edge in mst.raw_edges() {
            let source = edge.source();
            let target = edge.target();

            let source_tree = match map.get(&source) {
                Some(item) => item.clone(),
                None => {
                    let node = mst.node_weight(source).unwrap().clone();

                    let rbn = Rc::new(RefCell::new(RelationsBetweenNodes::new(node)));
                    map.insert(source, rbn.clone());

                    rbn
                }
            };

            let target_tree = match map.get(&target) {
                Some(item) => item.clone(),
                None => {
                    let node = mst.node_weight(target).unwrap().clone();

                    let rbn = Rc::new(RefCell::new(RelationsBetweenNodes::new(node)));
                    map.insert(target, rbn.clone());

                    rbn
                }
            };

            target_tree.borrow_mut().add_children(
                source_tree,
                self.named_links.get(&Link::new(source, target)).unwrap(),
            );

            map.insert(target, target_tree);
        }

        map.get(&self.output_node())
            .map(|item| item.borrow().clone())
    }

    pub fn generate(&self, width: u32, height: u32) -> Result<Plane> {
        let size = (width, height);
        let mut plane = Plane::new(size.0, size.1)?;

        let nodes_as_tree = self.nodes_as_tree().unwrap();

        for x in 0..size.0 {
            for y in 0..size.1 {
                let value =
                    nodes_as_tree.generate(&Coordinate::new_xy(x as f64, y as f64), &size)?;

                plane.put_pixel_unchecked(x as u32, y as u32, value.to_common_ground()?);
            }
        }

        Ok(plane)
    }
}

#[derive(Debug, Clone)]
struct RelationsBetweenNodes {
    node: Rc<RefCell<dyn Node>>,
    children: Vec<(Rc<RefCell<RelationsBetweenNodes>>, String)>,
}

impl RelationsBetweenNodes {
    fn new(node: Rc<RefCell<dyn Node>>) -> Self {
        RelationsBetweenNodes {
            node,
            children: Vec::new(),
        }
    }

    fn add_children<S: Into<String>>(&mut self, node: Rc<RefCell<RelationsBetweenNodes>>, name: S) {
        self.children.push((node, name.into()))
    }

    fn generate(&self, position: &Coordinate, size: &(u32, u32)) -> Result<InputOutputValue> {
        let mut children_results = HashMap::new();

        for (child_node, child_name) in &self.children {
            let out = child_node.borrow().generate(position, size)?;

            if self.node.borrow().is_output() {
                return Ok(out);
            }

            children_results.insert(child_name.clone(), out);
        }

        self.node
            .borrow()
            .generate(position, size, children_results)
    }
}

#[cfg(test)]
mod tests {
    use rusvid_core::pixel::Pixel;

    use super::Generator;
    use crate::coordinate::Coordinate;
    use crate::input_output_value::InputOutputValue;
    use crate::library::invert::Invert;
    use crate::library::mix::Mix;
    use crate::library::noise::Noise;
    use crate::library::static_value::StaticValue;
    use crate::link::Link;
    use crate::node::Node;

    #[test]
    fn a_node_can_have_more_than_one_inputs() {
        let mut g = Generator::new();

        let node_mix = g.add_node(Mix::new());
        let node_noise = g.add_node({
            let mut n = Noise::new(1);

            let scale = 10.0;

            n.set_scale(Coordinate::new_xy(scale, scale));

            n
        });
        let node_input1 = g.add_node(StaticValue::new(InputOutputValue::Pixel(Pixel::new(
            255, 0, 100, 255,
        ))));
        let node_input2 = g.add_node(StaticValue::new(InputOutputValue::Pixel(Pixel::new(
            0, 255, 150, 255,
        ))));
        let node_output = g.output_node();

        g.add_edge_named(Link::new(node_noise, node_mix), "value");
        g.add_edge_named(Link::new(node_input1, node_mix), "input1");
        g.add_edge_named(Link::new(node_input2, node_mix), "input2");
        g.add_edge(Link::new(node_mix, node_output));

        let plane = g.generate(10, 10).unwrap();

        // TODO maybe add a crate for snapshot testing
        assert_eq!(
            plane.as_data_flatten(),
            vec![
                0, 255, 150, 255, 3, 251, 149, 255, 0, 255, 161, 255, 0, 255, 158, 254, 143, 111,
                121, 255, 14, 240, 147, 255, 36, 218, 142, 254, 0, 255, 153, 255, 32, 222, 143,
                254, 0, 255, 150, 255, 0, 255, 155, 254, 35, 219, 143, 255, 0, 255, 156, 254, 0,
                255, 156, 255, 44, 210, 141, 255, 0, 255, 178, 255, 119, 135, 126, 255, 0, 255,
                164, 255, 0, 255, 155, 255, 3, 251, 149, 255, 0, 255, 158, 254, 12, 242, 147, 255,
                39, 215, 142, 255, 76, 178, 134, 255, 0, 255, 150, 255, 0, 255, 151, 254, 0, 255,
                156, 255, 0, 255, 164, 254, 46, 208, 140, 255, 0, 255, 158, 254, 0, 255, 157, 255,
                98, 156, 130, 255, 0, 255, 160, 254, 0, 255, 151, 255, 161, 93, 118, 255, 59, 195,
                138, 255, 36, 218, 142, 255, 51, 203, 139, 254, 0, 255, 173, 255, 0, 255, 157, 255,
                0, 255, 152, 255, 0, 255, 168, 255, 0, 255, 159, 255, 0, 255, 155, 255, 34, 220,
                143, 255, 0, 255, 157, 254, 105, 149, 129, 255, 23, 231, 145, 255, 69, 185, 136,
                255, 0, 255, 152, 255, 14, 240, 147, 255, 148, 106, 120, 255, 78, 176, 134, 255, 0,
                255, 160, 255, 222, 32, 106, 255, 0, 255, 173, 254, 0, 255, 164, 255, 0, 255, 160,
                255, 0, 255, 168, 255, 14, 240, 147, 255, 0, 255, 173, 255, 0, 255, 171, 255, 0,
                255, 158, 255, 60, 194, 138, 255, 115, 139, 127, 255, 85, 169, 133, 255, 0, 255,
                156, 254, 121, 133, 126, 255, 0, 255, 157, 255, 36, 218, 142, 254, 77, 177, 134,
                255, 77, 177, 134, 255, 66, 188, 137, 255, 0, 255, 171, 255, 121, 133, 126, 255, 0,
                255, 154, 255, 0, 255, 156, 255, 0, 255, 155, 254, 0, 255, 164, 255, 42, 212, 141,
                255, 0, 255, 156, 254, 0, 255, 150, 254, 29, 225, 144, 255, 71, 183, 135, 255, 0,
                255, 163, 254, 0, 255, 168, 255, 11, 243, 147, 255, 30, 224, 144, 255, 63, 191,
                137, 255, 0, 255, 150, 254, 0, 255, 150, 255, 32, 222, 143, 255, 17, 237, 146, 255,
                0, 255, 165, 255, 0, 255, 162, 255, 64, 190, 137, 255, 0, 255, 150, 255, 0, 255,
                150, 255, 0, 255, 150, 255, 0, 255, 150, 255,
            ]
        );
    }

    #[test]
    fn complex_nodes_graph() {
        let mut generator = Generator::new();

        let node_mix = generator.add_node({
            let mut m = Mix::new();
            m.space_info_mut().name = "Mix".to_string();
            m
        });
        let node_noise = generator.add_node({
            let mut n = Noise::new(1);

            let scale = 10.0;
            n.set_scale(Coordinate::new_xy(scale, scale));

            n.space_info_mut().name = "Noise".to_string();

            n
        });
        let node_input1 = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(255, 0, 100, 255),
        )));
        let node_input2 = generator.add_node(StaticValue::new(InputOutputValue::Pixel(
            Pixel::new(0, 255, 150, 255),
        )));
        let node_invert1 = generator.add_node(Invert::new());
        let node_invert2 = generator.add_node(Invert::new());
        let node_invert3 = generator.add_node(Invert::new());
        let node_output = generator.output_node();

        generator.add_edge(Link::new(node_noise, node_invert1));
        generator.add_edge(Link::new(node_invert1, node_invert2));
        generator.add_edge(Link::new(node_invert2, node_invert3));
        generator.add_edge_named(Link::new(node_invert3, node_mix), "value");
        generator.add_edge_named(Link::new(node_input1, node_mix), "input1");
        generator.add_edge_named(Link::new(node_input2, node_mix), "input2");
        generator.add_edge(Link::new(node_mix, node_output));

        let plane = generator.generate(10, 10).unwrap();

        assert!(plane
            .as_data_flatten()
            .iter()
            .zip([
                255, 0, 100, 255, 253, 1, 100, 255, 255, 0, 100, 255, 255, 0, 100, 255, 112, 143,
                128, 255, 241, 14, 102, 255, 219, 35, 107, 255, 255, 0, 100, 255, 223, 32, 106,
                255, 255, 0, 100, 255, 255, 0, 100, 255, 221, 33, 106, 255, 255, 0, 100, 255, 255,
                0, 100, 255, 211, 44, 108, 255, 255, 0, 100, 255, 136, 119, 123, 255, 255, 0, 100,
                255, 255, 0, 100, 255, 253, 1, 100, 255, 255, 0, 100, 255, 243, 12, 102, 255, 216,
                39, 107, 255, 179, 76, 114, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255, 0, 100,
                255, 255, 0, 100, 255, 209, 46, 109, 255, 255, 0, 100, 255, 255, 0, 100, 255, 158,
                97, 119, 255, 255, 0, 100, 255, 255, 0, 100, 255, 94, 161, 131, 255, 195, 60, 111,
                255, 219, 35, 107, 255, 205, 49, 109, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255,
                0, 100, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255, 0, 100, 255, 222, 32, 106,
                255, 255, 0, 100, 255, 150, 105, 120, 255, 232, 23, 104, 255, 186, 68, 113, 255,
                255, 0, 100, 255, 241, 14, 102, 255, 106, 148, 129, 255, 177, 78, 115, 255, 255, 0,
                100, 255, 32, 223, 143, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255, 0, 100, 255,
                255, 0, 100, 255, 241, 14, 102, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255, 0,
                100, 255, 195, 60, 111, 255, 141, 113, 122, 255, 170, 84, 116, 255, 255, 0, 100,
                255, 134, 121, 123, 255, 255, 0, 100, 255, 219, 35, 107, 255, 178, 77, 115, 255,
                178, 77, 115, 255, 190, 65, 112, 255, 255, 0, 100, 255, 134, 121, 123, 255, 255, 0,
                100, 255, 255, 0, 100, 255, 255, 0, 100, 255, 255, 0, 100, 255, 214, 40, 108, 255,
                255, 0, 100, 255, 255, 0, 100, 255, 226, 29, 105, 255, 184, 71, 113, 255, 255, 0,
                100, 255, 255, 0, 100, 255, 245, 9, 101, 255, 225, 30, 105, 255, 192, 63, 112, 255,
                255, 0, 100, 255, 255, 0, 100, 255, 223, 32, 106, 255, 239, 16, 103, 255, 255, 0,
                100, 255, 255, 0, 100, 255, 191, 64, 112, 255, 255, 0, 100, 255, 255, 0, 100, 255,
                255, 0, 100, 255, 255, 0, 100, 255,
            ])
            .map(|(calc, stored)| (stored as i32 - *calc as i32).abs() as u32)
            .fold(true, |acc, delta| delta <= 5 && acc));
    }
}
