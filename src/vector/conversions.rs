use super::Vector3;
use std::fmt::{Display, Formatter, Result};

impl Default for Vector3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<f32> for Vector3 {
    fn from(val: f32) -> Self {
        Self::splat(val)
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
