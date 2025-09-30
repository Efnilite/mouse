use std::fmt;

/// 2d vector with ints
#[derive(PartialEq, Copy, Clone)]
pub struct Vec2i {
    pub x: u8,
    pub y: u8
}

impl fmt::Debug for Vec2i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i({}, {})", self.x, self.y)?;
        Ok(())
    }
}

/// 2d vector with floats
#[derive(PartialEq, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl fmt::Debug for Vec2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f({}, {})", self.x, self.y)?;
        Ok(())
    }
}
