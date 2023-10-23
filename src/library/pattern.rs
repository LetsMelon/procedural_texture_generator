use std::collections::HashMap;

use anyhow::Result;
use rusvid_core::pixel::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct Pattern {
    space_info: SpaceInfo,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Pattern {
    fn generate(
        &self,
        position: &Coordinate,
        _size: &(u32, u32),
        input: HashMap<String, InputOutputValue>,
    ) -> Result<InputOutputValue> {
        let (_, first_input) = input.iter().next().unwrap();

        let p = first_input.to_common_ground()?;

        match (position.x() as usize % 2, position.y() as usize % 2) {
            (0, 0) | (1, 1) => Ok(InputOutputValue::Pixel(Pixel::new(
                ((p.get_r() as f64) * 0.0) as u8,
                ((p.get_g() as f64) * 0.0) as u8,
                ((p.get_b() as f64) * 0.0) as u8,
                255,
            ))),
            _ => Ok(InputOutputValue::Pixel(Pixel::new(
                ((p.get_r() as f64) * 1.0) as u8,
                ((p.get_g() as f64) * 1.0) as u8,
                ((p.get_b() as f64) * 1.0) as u8,
                255,
            ))),
        }
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
