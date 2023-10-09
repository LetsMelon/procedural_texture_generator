use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use petgraph::data::Build;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::{Directed, Graph};
use rusvid_core::plane::Plane;
use rusvid_core::prelude::Pixel;

use crate::bitmap::BitmapChar;
use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::link::Link;
use crate::node::Node;
use crate::render_square;

#[derive(Debug)]
pub struct SpaceNode {
    inner: Box<dyn Node>,

    pub position: (u32, u32),
    pub size: (u32, u32),
    pub color: Pixel,
    pub name: String,
    pub z_index: usize,
}

impl SpaceNode {
    pub fn new<N: Node + 'static>(node: N) -> Self {
        SpaceNode {
            inner: Box::new(node),
            position: (100, 100),
            size: (100, 100),
            color: Pixel::new(255, 100, 0, 255),
            // TODO name
            name: "node".to_string(),
            z_index: 0,
        }
    }

    pub fn render(&self, plane: &mut Plane) -> Result<()> {
        render_square(plane, self.position, self.size, self.color)?;
        BitmapChar::render_multiple_with_scale(
            plane,
            (self.position.0, self.position.1 + 5),
            &self.name,
            Pixel::new(0, 0, 0, 255),
            2,
        )?;

        Ok(())
    }

    pub fn add_delta(&mut self, delta: (i32, i32)) {}
}

impl Node for SpaceNode {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(u32, u32),
        input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        self.inner.generate(position, size, input)
    }
}

#[derive(Debug)]
pub struct Generator {
    pub(crate) internal_graph: Graph<Rc<RefCell<SpaceNode>>, (), Directed>,

    output_node: NodeIndex,
}

unsafe impl Sync for Generator {}
unsafe impl Send for Generator {}

impl Generator {
    pub fn new() -> Self {
        let mut g = Generator {
            internal_graph: Graph::new(),
            // TODO maybe use an option for this because we set this value in the following lines and '0' is a fake value
            output_node: NodeIndex::new(0),
        };

        g.output_node = g.add_node_with_space({
            let mut sn = SpaceNode::new(crate::library::output::Output);

            sn.name = "Output".to_string();
            sn.color = Pixel::new(166, 166, 166, 255);
            sn.size = (100, 30);

            sn
        });

        g
    }

    pub fn output_node(&self) -> NodeIndex {
        self.output_node
    }

    pub fn add_node<N: Node + 'static>(&mut self, node: N) -> NodeIndex {
        let node = SpaceNode::new(node);

        self.add_node_with_space(node)
    }

    pub fn add_node_with_space(&mut self, node: SpaceNode) -> NodeIndex {
        self.internal_graph.add_node(Rc::new(RefCell::new(node)))
    }

    pub fn add_edge(&mut self, link: Link) -> EdgeIndex {
        self.internal_graph
            .add_edge(link.input_node, link.output_node, ())
    }

    pub fn connected_nodes_to_output(&self) -> Vec<Rc<RefCell<SpaceNode>>> {
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

        for x in 0..size.0 {
            for y in 0..size.1 {
                let mut value = InputOutputValue::Nothing;

                let pos = Coordinate::new_xy(x as f64, y as f64);

                for node in &used_nodes_for_output {
                    let node = node.borrow();

                    // TODO change that to something else so that we don't allocate a new string all the time when checking if the node is an `Output`
                    if format!("{:?}", node).contains("Output") {
                        break;
                    }
                    value = node.generate(&pos, &size, value)?;
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
