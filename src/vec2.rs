/// 2d vector with ints
pub struct Vec2i {
    x: u8,
    y: u8,
}

impl Vec2i {
    /// Returns the Euclidean length of the vector as `f32`
    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
}

/// 2d vector with floats
pub struct Vec2f {
    x: f32,
    y: f32,
}

impl Vec2f {
    /// Returns the Euclidean length of the vector as `f32`
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2::{Vec2f, Vec2i};

    #[test]
    fn vec2i_length() {
        let vec = Vec2i { x: 1, y: 1 };
        assert_eq!(f32::sqrt(2f32), vec.length());

        let vec = Vec2i { x: 4, y: 4 };
        assert_eq!(f32::sqrt(32f32), vec.length());
    }

    #[test]
    fn vec2f_length() {
        let vec = Vec2f { x: 1f32, y: 1f32 };
        assert_eq!(f32::sqrt(2f32), vec.length());

        let vec = Vec2f { x: 3.5, y: 4f32 };
        assert_eq!(f32::sqrt(28.25), vec.length());
    }

}
