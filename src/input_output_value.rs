use anyhow::Result;
use rusvid_core::pixel::Pixel;

macro_rules! impl_from_t {
    ((), $value:expr) => {
        impl From<()> for InputOutputValue {
            fn from(_: ()) -> Self {
                $value
            }
        }
    };
    ($t:ty, $value:expr) => {
        impl From<$t> for InputOutputValue {
            fn from(value: $t) -> Self {
                $value(value)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputOutputValue {
    Nothing,
    Float(f64),
    Pixel(Pixel),
    U8X3Array([u8; 3]),
    U8X4Array([u8; 4]),
    F64X3Array([f64; 3]),
    F64X4Array([f64; 4]),
}

impl InputOutputValue {
    pub fn to_common_ground(&self) -> Result<Pixel> {
        match self {
            InputOutputValue::Nothing => Ok(Pixel::ZERO),
            InputOutputValue::Float(value) => {
                InputOutputValue::F64X4Array([*value, *value, *value, 1.0]).to_common_ground()
            }
            InputOutputValue::Pixel(value) => Ok(*value),
            InputOutputValue::U8X3Array(value) => {
                InputOutputValue::U8X4Array([value[0], value[1], value[2], 255]).to_common_ground()
            }
            InputOutputValue::U8X4Array(value) => {
                Ok(Pixel::new(value[0], value[1], value[2], value[3]))
            }
            InputOutputValue::F64X3Array(value) => {
                InputOutputValue::F64X4Array([value[0], value[1], value[2], 1.0]).to_common_ground()
            }
            InputOutputValue::F64X4Array(value) => Ok(Pixel::new(
                (value[0] * 255.0) as u8,
                (value[1] * 255.0) as u8,
                (value[2] * 255.0) as u8,
                (value[3] * 255.0) as u8,
            )),
        }
    }

    pub fn r_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_r() as f64) / 255.0)
    }

    pub fn g_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_g() as f64) / 255.0)
    }

    pub fn b_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_b() as f64) / 255.0)
    }

    pub fn a_percentage(&self) -> Result<f64> {
        let p = self.to_common_ground()?;

        Ok((p.get_a() as f64) / 255.0)
    }
}

impl_from_t!((), InputOutputValue::Nothing);
impl_from_t!(f64, InputOutputValue::Float);
impl_from_t!(Pixel, InputOutputValue::Pixel);
impl_from_t!([u8; 3], InputOutputValue::U8X3Array);
impl_from_t!([u8; 4], InputOutputValue::U8X4Array);
impl_from_t!([f64; 3], InputOutputValue::F64X3Array);
impl_from_t!([f64; 4], InputOutputValue::F64X4Array);
