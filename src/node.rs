use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::bitmap::BitmapChar;
use crate::coordinate::Coordinate;
use crate::input_output_value::InputOutputValue;
use crate::utils::render_square;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpaceInfo {
    pub position: (i64, i64),
    pub size: (u32, u32),
    pub color: Pixel,
    pub name: String,
    pub z_index: usize,
}

impl Default for SpaceInfo {
    fn default() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Self {
            position: (100, 100),
            size: (100, 100),
            color: Pixel::new(255, 100, 0, 255),
            name: format!("node_{}", COUNTER.fetch_add(1, Ordering::Relaxed)),
            z_index: 0,
        }
    }
}

pub trait Node: std::fmt::Debug {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(u32, u32),
        input: HashMap<String, InputOutputValue>,
    ) -> Result<InputOutputValue>;

    fn is_output(&self) -> bool {
        false
    }

    fn render(&self, plane: &mut Plane) -> Result<()> {
        let space_info = self.space_info();

        render_square(
            plane,
            space_info.position,
            space_info.size,
            space_info.color,
        )?;
        BitmapChar::render_multiple_with_scale(
            plane,
            (space_info.position.0, space_info.position.1 + 5),
            &space_info.name,
            Pixel::new(0, 0, 0, 255),
            2,
        )?;

        Ok(())
    }

    fn space_info(&self) -> &SpaceInfo;
    fn space_info_mut(&mut self) -> &mut SpaceInfo;
}
