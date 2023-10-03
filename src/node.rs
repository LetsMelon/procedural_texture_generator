use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;

pub trait Node: std::fmt::Debug {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(u32, u32),
        input: InputOutputValue,
    ) -> Result<InputOutputValue>;
}
