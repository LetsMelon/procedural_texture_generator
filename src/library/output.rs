use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Output;

impl Node for Output {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        _input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        todo!()
    }
}
