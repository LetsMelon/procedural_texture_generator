use anyhow::Result;
use rusvid_core::prelude::Pixel;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct Map {
    // TODO maybe use `Range` instead of `f64` and use a separate struct
    steps: Vec<(InputOutputValue, f64)>,
}

impl Map {
    pub fn new(steps: Vec<(InputOutputValue, f64)>) -> Self {
        assert!(steps.len() >= 2);

        Map { steps }
    }
}

impl Node for Map {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        let r = input.r_percentage()?;
        let g = input.g_percentage()?;
        let b = input.b_percentage()?;
        let avg = (r + g + b) / 3.0;

        let mut first = self.steps.first().unwrap();
        let mut last = self.steps.last().unwrap();

        if self.steps.len() != 2 {
            for step in &self.steps {
                if step.1 < avg {
                    first = step;
                } else {
                    last = step;
                    break;
                }
            }
        }

        if avg < first.1 {
            Ok(first.0)
        } else if avg >= last.1 {
            Ok(last.0)
        } else {
            let v1 = [
                first.0.r_percentage()?,
                first.0.g_percentage()?,
                first.0.b_percentage()?,
                first.0.a_percentage()?,
            ];

            let v2 = [
                last.0.r_percentage()?,
                last.0.g_percentage()?,
                last.0.b_percentage()?,
                last.0.a_percentage()?,
            ];

            let span_delta = last.1 - first.1;
            let first_delta = avg - first.1;
            let percentage = first_delta / span_delta;

            let values = v1
                .iter()
                .zip(v2.iter())
                // .map(|(v1, v2)| (((v1 * percentage + v2 * (1.0 - percentage)) * 255.0) as u8))
                .map(|(v1, v2)| (((v1 * (1.0 - percentage) + v2 * percentage) * 255.0) as u8))
                .collect::<Vec<_>>();

            let arr = [values[0], values[1], values[2], values[3]];

            Ok(InputOutputValue::Pixel(Pixel::new_raw(arr)))
        }
    }
}
