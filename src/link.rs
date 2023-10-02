use std::cell::RefCell;
use std::rc::Rc;

use petgraph::stable_graph::NodeIndex;

use crate::node::Node;

pub type LinkNode = Rc<RefCell<dyn Node>>;

#[derive(Debug, Clone)]
pub struct Link {
    pub(crate) input_node: NodeIndex,
    pub(crate) output_node: NodeIndex,
    // TODO cache the output from `input_node` so that we don't have to recalculate in every link
}

impl Link {
    pub fn new(input_node: NodeIndex, output_node: NodeIndex) -> Self {
        assert_ne!(
            input_node, output_node,
            "Input and output node are not allowed to be the same"
        );

        Link {
            input_node,
            output_node,
        }
    }
}
