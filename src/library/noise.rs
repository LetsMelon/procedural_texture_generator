use anyhow::Result;
use noise::{NoiseFn, Perlin};

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::Node;

#[derive(Debug)]
pub struct Noise {
    perlin: Perlin,
    offset: Coordinate,
    scale: Coordinate,
}

impl Noise {
    pub fn new(seed: u32) -> Self {
        Noise {
            perlin: Perlin::new(seed),
            offset: Coordinate::new(0.0, 0.0, 0.0),
            scale: Coordinate::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_offset(&mut self, offset: Coordinate) {
        self.offset = offset;
    }

    pub fn set_scale(&mut self, scale: Coordinate) {
        self.scale = scale;
    }
}

impl Node for Noise {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(usize, usize),
        _input: Box<dyn InputOutputValue>,
    ) -> Result<Box<dyn InputOutputValue>> {
        let value = self.perlin.get([
            (((position.x() as f64) / ((size.0 - 1) as f64)) + self.offset.x()) * self.scale.x(),
            (((position.y() as f64) / ((size.1 - 1) as f64)) + self.offset.y()) * self.scale.y(),
            ((position.z() as f64) + self.offset.z() as f64) * self.scale.z(),
        ]);

        Ok(Box::new(value))
    }
}
