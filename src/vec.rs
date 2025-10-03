use std::fmt;

/// 2d vector with ints
#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct Veci {
    pub x: u8,
    pub y: u8,
}

impl Veci {
    /// Returns a new [Veci] as zero-vector.
    pub(crate) fn new() -> Self {
        Veci { x: 0, y: 0 }
    }
}

impl fmt::Debug for Veci {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i({}, {})", self.x, self.y)?;
        Ok(())
    }
}

/// 2d vector with floats
#[derive(PartialEq, Copy, Clone)]
pub struct Vecf {
    pub x: f32,
    pub y: f32,
}

impl Vecf {
    /// Returns a new [Vecf] as zero-vector.
    fn new() -> Self {
        Vecf { x: 0f32, y: 0f32 }
    }
}

impl fmt::Debug for Vecf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f({}, {})", self.x, self.y)?;
        Ok(())
    }
}
