use anyhow::Result;
use rusvid_core::prelude::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct Mix {
    space_info: SpaceInfo,
}

impl Mix {
    pub fn new() -> Self {
        Mix {
            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Mix {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: &[InputOutputValue],
    ) -> Result<InputOutputValue> {
        assert_eq!(input.len(), 3);

        // `delta` must be from type `InputOutputValue::Float`
        let deltas = match input[2] {
            InputOutputValue::Float(value) => [value, value, value, value],
            InputOutputValue::Nothing
            | InputOutputValue::Pixel(_)
            | InputOutputValue::U8X3Array(_)
            | InputOutputValue::U8X4Array(_) => todo!(),
            InputOutputValue::F64X3Array(values) => [values[0], values[1], values[2], 1.0],
            InputOutputValue::F64X4Array(values) => [values[0], values[1], values[2], values[3]],
        };

        let values = input[0]
            .to_common_ground()?
            .to_raw_float()
            .map(|item| item as f64)
            .zip(
                input[1]
                    .to_common_ground()?
                    .to_raw_float()
                    .map(|item| item as f64),
            )
            .zip(deltas)
            .map(|((first, second), delta)| (first * delta + second * (1.0 - delta)) as u8)
            .collect::<Vec<_>>();

        Ok(InputOutputValue::Pixel(Pixel::new(
            values[0], values[1], values[2], values[3],
        )))
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
