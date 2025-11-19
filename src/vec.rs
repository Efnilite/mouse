/// 2d vector with u8s
#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct Vecu {
    pub x: u8,
    pub y: u8,
}

impl Vecu {
    /// Returns a new [Vecu] as zero-vector.
    pub fn new() -> Self {
        Vecu { x: 0, y: 0 }
    }
}

impl Default for Vecu {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Vecu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)?;
        Ok(())
    }
}

/// 2d vector with u8s
#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct Veci {
    pub x: i16,
    pub y: i16,
}

impl Veci {
    /// Returns a new [Vecu] as zero-vector.
    pub fn new() -> Self {
        Veci { x: 0, y: 0 }
    }
}

impl Default for Veci {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Veci {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)?;
        Ok(())
    }
}

/// 2d vector with floats
#[derive(PartialEq, Copy, Clone)]
pub struct Vecf {
    pub x: f64,
    pub y: f64,
}

impl Vecf {
    /// Returns a new [Vecf] as zero-vector.
    pub fn new() -> Self {
        Vecf { x: 0f64, y: 0f64 }
    }

    /// Returns the distance to `other`.
    pub fn distance(&self, other: &Vecf) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        f64::sqrt(dx * dx + dy * dy)
    }

    /// Returns the length of this vector.
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x + self.y)
    }

    /// Normalizes this vector to a length of one.
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
    }

    /// Rotates counter-clockwise over `a` radians at the origin.
    pub fn rotate(&mut self, a: f64) {
        let x = self.x * f64::cos(a) - self.y * f64::sin(a);
        let y = self.x * f64::sin(a) + self.y * f64::cos(a);
        self.x = x;
        self.y = y;
    }

    /// Adds a `Vecf` to the current vec.
    pub fn add(&mut self, vec: Vecf) {
        self.x = self.x + vec.x;
        self.y = self.y + vec.y;
    }

}

impl Default for Vecf {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Vecf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)?;
        Ok(())
    }
}
