pub use crate::vec3::Vec3 as Color;
use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.x()) as u64,
            (255.999 * self.y()) as u64,
            (255.999 * self.z()) as u64,
        )
    }
}
