pub use crate::vec3::Vec3 as Point;

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr ) => {
        Point::new([$x, $y, $z])
    };
}
