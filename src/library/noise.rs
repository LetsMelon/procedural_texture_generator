use anyhow::Result;
use noise::{NoiseFn, Perlin};
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::bitmap::BitmapChar;
use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};
use crate::render_square;

#[derive(Debug)]
pub struct Noise {
    perlin: Perlin,

    seed: u32,
    offset: Coordinate,
    scale: Coordinate,

    space_info: SpaceInfo,
}

impl Noise {
    pub fn new(seed: u32) -> Self {
        Noise {
            perlin: Perlin::new(seed),
            seed,
            offset: Coordinate::new(0.0, 0.0, 0.0),
            scale: Coordinate::new(1.0, 1.0, 1.0),

            space_info: {
                let mut si = SpaceInfo::default();

                si.size = (250, 50);

                si
            },
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
        size: &(u32, u32),
        _input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        let value = self.perlin.get([
            (((position.x()) / ((size.0 - 1) as f64)) + self.offset.x()) * self.scale.x(),
            (((position.y()) / ((size.1 - 1) as f64)) + self.offset.y()) * self.scale.y(),
            ((position.z()) + self.offset.z()) * self.scale.z(),
        ]);

        Ok(InputOutputValue::Float(value))
    }

    fn render(&self, plane: &mut Plane) -> Result<()> {
        let space_info = self.space_info();

        render_square(
            plane,
            space_info.position,
            space_info.size,
            space_info.color,
        )?;

        let mut delta_y = 0;
        let (_, delta) = BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5),
            &space_info.name,
            Pixel::new(0, 0, 0, 255),
            2,
        )?;
        delta_y += delta;

        let (_, delta) = BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5 + delta_y),
            format!("Seed: {}", self.seed),
            Pixel::new(0, 0, 0, 255),
            1,
        )?;
        delta_y += delta;

        let (_, delta) = BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5 + delta_y),
            format!("Offset: {}", self.offset),
            Pixel::new(0, 0, 0, 255),
            1,
        )?;
        delta_y += delta;

        BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5 + delta_y),
            format!("Scale: {}", self.scale),
            Pixel::new(0, 0, 0, 255),
            1,
        )?;

        Ok(())
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
