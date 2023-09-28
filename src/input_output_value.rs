use anyhow::Result;
use rusvid_core::pixel::Pixel;

pub trait InputOutputValue: std::fmt::Debug {
    fn to_common_ground(&self) -> Result<Pixel>;

    fn r_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_r() as f64) / 255.0)
    }

    fn g_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_g() as f64) / 255.0)
    }

    fn b_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_b() as f64) / 255.0)
    }

    fn a_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_a() as f64) / 255.0)
    }
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
        Ok(*self)
    }
}

impl InputOutputValue for [u8; 3] {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(self[0], self[1], self[2], 255))
    }
}

impl InputOutputValue for [u8; 4] {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(self[0], self[1], self[2], self[3]))
    }
}

impl InputOutputValue for [f64; 3] {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(
            (self[0] * 255.0) as u8,
            (self[1] * 255.0) as u8,
            (self[2] * 255.0) as u8,
            255,
        ))
    }
}

impl InputOutputValue for [f64; 4] {
    fn to_common_ground(&self) -> Result<Pixel> {
        Ok(Pixel::new(
            (self[0] * 255.0) as u8,
            (self[1] * 255.0) as u8,
            (self[2] * 255.0) as u8,
            (self[3] * 255.0) as u8,
        ))
    }
}
