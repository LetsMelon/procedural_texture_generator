use anyhow::Result;
use rusvid_core::prelude::Pixel;

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
        space_info.size = (100, 30);

        Output { space_info }
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
