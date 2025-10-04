/// 2d vector with ints
#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
pub struct Veci {
    pub x: u8,
    pub y: u8,
}

impl Veci {
    /// Returns a new [Veci] as zero-vector.
    pub fn new() -> Self {
        Veci { x: 0, y: 0 }
    }
}

/// 2d vector with floats
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vecf {
    pub x: f32,
    pub y: f32,
}

impl Vecf {
    /// Returns a new [Vecf] as zero-vector.
    pub fn new() -> Self {
        Vecf { x: 0f32, y: 0f32 }
    }
}
