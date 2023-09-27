#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    #[inline(always)]
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Coordinate { x, y, z }
    }

    #[inline(always)]
    pub fn new_x(x: usize) -> Self {
        Self::new_xy(x, 0)
    }

    #[inline(always)]
    pub fn new_xy(x: usize, y: usize) -> Self {
        Self::new(x, y, 0)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }
}
