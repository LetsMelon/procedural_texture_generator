use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::{Directed, Graph};
use rusvid_core::plane::Plane;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::link::Link;
use crate::node::Node;

#[derive(Debug)]
pub struct Generator {
    internal_graph: Graph<Rc<RefCell<dyn Node>>, (), Directed>,

    output_node: NodeIndex,
}

impl Generator {
    pub fn new() -> Self {
        let mut g = Generator {
            internal_graph: Graph::new(),
            // TODO maybe use an option for this because we set this value in the following lines and '0' is a fake value
            output_node: NodeIndex::new(0),
        };

        g.output_node = g.add_node(crate::library::output::Output);

        g
    }

    pub fn output_node(&self) -> NodeIndex {
        self.output_node
    }

    pub fn add_node<N: Node + 'static>(&mut self, node: N) -> NodeIndex {
        let node = Rc::new(RefCell::new(node));

        self.internal_graph.add_node(node)
    }

    pub fn add_edge(&mut self, link: Link) -> EdgeIndex {
        self.internal_graph
            .add_edge(link.input_node, link.output_node, ())
    }

    pub fn generate(&self) -> Result<Plane> {
        let side = 100;
        let size = (side, side);
        let mut plane = Plane::new(size.0 as u32, size.1 as u32)?;

        let mut g = self.internal_graph.clone();
        g.reverse();

        let mut used_nodes_for_output = Vec::with_capacity(g.node_count());
        let mut dfs = Dfs::new(&g, self.output_node());
        while let Some(nx) = dfs.next(&g) {
            used_nodes_for_output.push(g[nx].clone());
        }
        used_nodes_for_output.reverse();

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
