pub use crate::vec3::Vec3 as Color;
use std::fmt;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr ) => {
        $crate::color::Color::new([$x, $y, $z])
    };
}

impl Color {
    pub(crate) fn write(&self, samples_per_pixel: u32) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        format!(
            "{} {} {} ",
            (256.0 * (self.x() * scale).sqrt().clamp(0.0, 0.999)) as u8,
            (256.0 * (self.y() * scale).sqrt().clamp(0.0, 0.999)) as u8,
            (256.0 * (self.z() * scale).sqrt().clamp(0.0, 0.999)) as u8,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self.x().clamp(0.0, 0.999)) as u8,
            (256.0 * self.y().clamp(0.0, 0.999)) as u8,
            (256.0 * self.z().clamp(0.0, 0.999)) as u8,
        )
    }
}
