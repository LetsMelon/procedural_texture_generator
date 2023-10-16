use anyhow::Result;
use rusvid_core::prelude::{Pixel, Plane};

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

// TODO implement this method into crate `rusvid_core`
fn copy_plane(
    destination: &mut Plane,
    source: &Plane,
    position: (u32, u32),
    size: (u32, u32),
) -> Result<()> {
    let source_width = source.width() as f64 - 1.0;
    let source_height = source.height() as f64 - 1.0;

    let copy_width = size.0 as f64 - 1.0;
    let copy_height = size.1 as f64 - 1.0;

    for destination_delta_x in 0..size.0 {
        for destination_delta_y in 0..size.1 {
            let destination_x = position.0 + destination_delta_x;
            let destination_y = position.1 + destination_delta_y;

            let source_x = (destination_delta_x as f64 / copy_width * source_width).round() as u32;
            let source_y =
                (destination_delta_y as f64 / copy_height * source_height).round() as u32;

            let source_pixel = source.pixel(source_x, source_y).unwrap();
            destination.put_pixel(destination_x, destination_y, source_pixel.clone())?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    space_info: SpaceInfo,
}

impl Output {
    pub fn new() -> Self {
        let mut space_info = SpaceInfo::default();
        space_info.name = "Output".to_string();
        space_info.color = Pixel::new(166, 166, 166, 255);
        space_info.size = (220, 235);

        Output { space_info }
    }

    pub fn draw_generated_output_into_node(
        &self,
        plane: &mut Plane,
        generated_plane: &Plane,
    ) -> Result<()> {
        copy_plane(
            plane,
            generated_plane,
            {
                let pos = self.space_info.position;

                (pos.0 as u32 + 10, pos.1 as u32 + 25)
            },
            (200, 200),
        )?;

        Ok(())
    }
}

impl Node for Output {
    fn generate(
        &self,
        _position: &Coordinate,
        _size: &(u32, u32),
        _input: InputOutputValue,
    ) -> Result<InputOutputValue> {
        todo!()
    }

    fn is_output(&self) -> bool {
        true
    }

    fn space_info(&self) -> &SpaceInfo {
        &self.space_info
    }

    fn space_info_mut(&mut self) -> &mut SpaceInfo {
        &mut self.space_info
    }
}
