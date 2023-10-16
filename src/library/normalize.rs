use anyhow::Result;
use rusvid_core::prelude::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

#[derive(Debug)]
pub struct Normalize {
    value: f64,

    space_info: SpaceInfo,
}

impl Normalize {
    pub fn new(value: f64) -> Self {
        Normalize {
            // TODO use '.clamp' instead of '.min' and '.max'
            value: value.min(1.0).max(0.0001),

            space_info: SpaceInfo::default(),
        }
    }
}

impl Node for Normalize {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        let raw = input.to_common_ground()?.to_raw();

        let values = raw
            .iter()
            .map(|item| 255.0 / (*item as f64))
            // TODO use '.clamp' instead of '.min' and '.max'
            .map(|item| ((item / self.value).min(1.0).max(0.0) * 255.0) as u8)
            .collect::<Vec<_>>();

        dbg!(&values);

        let arr = [values[0], values[1], values[2], values[3]];
        Ok(InputOutputValue::Pixel(Pixel::new_raw(arr)))
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
