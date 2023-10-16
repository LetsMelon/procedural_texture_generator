use anyhow::Result;
use rusvid_core::prelude::{Pixel, Plane, ResizeMode};

use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::node::{Node, SpaceInfo};

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
        let pos = self.space_info.position;
        plane.copy_into(
            generated_plane,
            pos.0 as u32 + 10,
            pos.1 as u32 + 25,
            200,
            200,
            ResizeMode::NearestNeighbor,
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
