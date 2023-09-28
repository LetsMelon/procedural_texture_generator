use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;

pub trait Node: std::fmt::Debug {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(usize, usize),
        input: Box<dyn InputOutputValue>,
    ) -> Result<Box<dyn InputOutputValue>>;
}
