use anyhow::Result;
use rusvid_core::pixel::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct BlankColor {
    pub color: Pixel,
}

impl Node for BlankColor {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(usize, usize),
        _input: Box<dyn InputOutputValue>,
    ) -> Result<Box<dyn InputOutputValue>> {
        Ok(Box::new(self.color))
    }
}
