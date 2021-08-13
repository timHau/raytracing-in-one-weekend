pub use crate::vec3::Vec3 as Color;
use std::fmt;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr ) => {
        $crate::color::Color::new([$x, $y, $z])
    };
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.x()) as i64,
            (255.999 * self.y()) as i64,
            (255.999 * self.z()) as i64,
        )
    }
}
