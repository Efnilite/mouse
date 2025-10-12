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
}

impl core::fmt::Debug for Vecf {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)?;
        Ok(())
    }

}