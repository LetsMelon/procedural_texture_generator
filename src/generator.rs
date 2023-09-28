use anyhow::Result;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct Generator {
    pub nodes: Vec<Box<dyn Node>>,
}

impl Generator {
    pub fn generate(&self) -> Result<Plane> {
        let side = 1000;
        let size = (side, side);

        let mut plane = Plane::new(size.0 as u32, size.1 as u32)?;

        for x in 0..size.0 {
            for y in 0..size.0 {
                let mut value = InputOutputValue::Pixel(Pixel::ZERO);

                for node in &self.nodes {
                    let p = node.generate(&Coordinate::new_xy(x as f64, y as f64), &size, value)?;

                    value = p;
                }

                plane.put_pixel_unchecked(x as u32, y as u32, value.to_common_ground()?);
            }
        }

        Ok(plane)
    }
}
