use anyhow::Result;
use rusvid_core::pixel::Pixel;

pub trait InputOutputValue: std::fmt::Debug {
    fn to_common_ground(&self) -> Result<Pixel>;
}

impl InputOutputValue for () {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::ZERO)
    }
}

impl InputOutputValue for f32 {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(
            (255.0 * self) as u8,
            (255.0 * self) as u8,
            (255.0 * self) as u8,
            255,
        ))
    }
}

impl InputOutputValue for f64 {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(
            (255.0 * self) as u8,
            (255.0 * self) as u8,
            (255.0 * self) as u8,
            255,
        ))
    }
}

impl InputOutputValue for Pixel {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(self.clone())
    }
}
