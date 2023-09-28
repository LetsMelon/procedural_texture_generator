#[derive(Debug)]
pub struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate {
    #[inline(always)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Coordinate { x, y, z }
    }

    #[inline(always)]
    pub fn new_x(x: f64) -> Self {
        Self::new_xy(x, 0.0)
    }

    #[inline(always)]
    pub fn new_xy(x: f64, y: f64) -> Self {
        Self::new(x, y, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}
