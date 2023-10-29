use std::collections::HashMap;

use anyhow::Result;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct Invert {
    space_info: SpaceInfo,
}

impl Invert {
    pub fn new() -> Self {
        Invert {
            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Invert {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: HashMap<String, InputOutputValue>,
    ) -> Result<InputOutputValue> {
        let (_, first_input) = input.iter().next().unwrap();

        let values = first_input
            .percentages()?
            .iter()
            .map(|item| 1.0 - *item)
            .collect::<Vec<_>>();

        Ok(InputOutputValue::F64X4Array([
            values[0], values[1], values[2], values[3],
        ]))
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
