use anyhow::Result;
use rusvid_core::prelude::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct Normalize {
    value: f64,
}

impl Normalize {
    pub fn new(value: f64) -> Self {
        Normalize {
            value: value.min(1.0).max(0.0001),
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
            .map(|item| ((item / self.value).min(1.0).max(0.0) * 255.0) as u8)
            .collect::<Vec<_>>();

        dbg!(&values);

        let arr = [values[0], values[1], values[2], values[3]];
        Ok(InputOutputValue::Pixel(Pixel::new_raw(arr)))
    }
}
