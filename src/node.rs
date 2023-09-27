use anyhow::Result;

use crate::{Coordinate, InputOutputValue};

pub trait Node: std::fmt::Debug {
    fn generate(
        &self,
        position: &Coordinate,
        size: &(usize, usize),
        input: Box<dyn InputOutputValue>,
    ) -> Result<Box<dyn InputOutputValue>>;
}
