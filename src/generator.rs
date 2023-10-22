use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::{Directed, Graph};
use rusvid_core::plane::Plane;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::library::output::Output;
use crate::link::Link;
use crate::node::{Node, SpaceInfo};

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

    pub fn add_edge(&mut self, link: Link) -> (EdgeIndex, String) {
        self.add_edge_named(link, "_")
    }

    pub fn add_edge_named<S: Into<String>>(&mut self, link: Link, name: S) -> (EdgeIndex, String) {
        let name: String = name.into();
        self.named_links.insert(link.clone(), name.clone());

        (
            self.internal_graph
                .add_edge(link.input_node, link.output_node, ()),
            name,
        )
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

    pub fn generate(&self, width: u32, height: u32) -> Result<Plane> {
        let size = (width, height);
        let mut plane = Plane::new(size.0, size.1)?;

        let used_nodes_for_output = self.connected_nodes_to_output();

        dbg!(&used_nodes_for_output);

        for x in 0..size.0 {
            for y in 0..size.1 {
                let mut value = InputOutputValue::Nothing;

                let pos = Coordinate::new_xy(x as f64, y as f64);

                for node in &used_nodes_for_output {
                    let node = node.borrow();

                    if node.is_output() {
                        break;
                    }
                    value = node.generate(&pos, &size, &[value])?;
                }

                plane.put_pixel_unchecked(x as u32, y as u32, value.to_common_ground()?);
            }
        }

        Ok(plane)
    }
}

/*
fn save_graph<P: AsRef<std::path::Path>, U: std::fmt::Debug, V: std::fmt::Debug>(
    g: &Graph<U, V>,
    path: P,
) -> Result<()> {
    use std::io::Write;

    let mut dot_child = std::process::Command::new("dot")
        .arg("-Tpng")
        .args(["-o", path.as_ref().to_str().unwrap()])
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    let child_stdin = dot_child.stdin.as_mut().unwrap();
    write!(
        child_stdin,
        "{:?}",
        petgraph::dot::Dot::with_config(g, &[petgraph::dot::Config::EdgeNoLabel])
    )?;
    let _ = child_stdin;

    let output = dot_child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Exited binary `dot` with an error.\nerr: {:?}",
            output.stderr
        )
    } else {
        Ok(())
    }
}
 */

/*
Output
    Mix
        StaticValue
        Noise
        StaticValue
*/
#[derive(Debug)]
pub struct RelationsBetweenNodes {
    node: Rc<RefCell<dyn Node>>,
    children: Vec<(RelationsBetweenNodes, String)>,
}

impl RelationsBetweenNodes {
    pub fn new(node: Rc<RefCell<dyn Node>>) -> Self {
        RelationsBetweenNodes {
            node,
            children: Vec::new(),
        }
    }

    pub fn add_children<S: Into<String>>(&mut self, node: RelationsBetweenNodes, name: S) {
        self.children.push((node, name.into()))
    }
}

impl Node for RelationsBetweenNodes {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(u32, u32),
        input: &[InputOutputValue],
    ) -> Result<InputOutputValue> {
        todo!()
    }

    fn space_info(&self) -> &SpaceInfo {
        todo!()
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use rusvid_core::prelude::Pixel;

    use super::RelationsBetweenNodes;
    use crate::input_output_value::InputOutputValue;
    use crate::library::noise::Noise;
    use crate::library::output::Output;
    use crate::library::static_value::StaticValue;

    #[test]
    fn just_works() {
        let rbn = RelationsBetweenNodes {
            node: Rc::new(RefCell::new(Output::new())),
            children: vec![
                (
                    RelationsBetweenNodes::new(Rc::new(RefCell::new(StaticValue::new(
                        InputOutputValue::Float(0.5),
                    )))),
                    "value".to_string(),
                ),
                (
                    RelationsBetweenNodes::new(Rc::new(RefCell::new(StaticValue::new(
                        InputOutputValue::Pixel(Pixel::new(255, 0, 100, 255)),
                    )))),
                    "input1".to_string(),
                ),
                (
                    RelationsBetweenNodes::new(Rc::new(RefCell::new(Noise::new(1)))),
                    "input2".to_string(),
                ),
            ],
        };
    }
}
