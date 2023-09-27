use anyhow::Result;
use rusvid_core::pixel::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct Pattern;

impl Node for Pattern {
    fn generate(
        &self,
        position: &Coordinate,
        _size: &(usize, usize),
        input: Box<dyn InputOutputValue>,
    ) -> Result<Box<dyn InputOutputValue>> {
        let p = input.to_common_ground()?;

        match (position.x() % 2, position.y() % 2) {
            (0, 0) | (1, 1) => Ok(Box::new(Pixel::new(
                ((p.get_r() as f64) * 0.0) as u8,
                ((p.get_g() as f64) * 0.0) as u8,
                ((p.get_b() as f64) * 0.0) as u8,
                255,
            ))),
            _ => Ok(Box::new(Pixel::new(
                ((p.get_r() as f64) * 1.0) as u8,
                ((p.get_g() as f64) * 1.0) as u8,
                ((p.get_b() as f64) * 1.0) as u8,
                255,
            ))),
        }
    }
}