use crate::{point::Point, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub(crate) struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point::new([0.0, 0.0, 0.0]);
        let horizontal = Vec3::new([viewport_width, 0.0, 0.0]);
        let vertical = Vec3::new([0.0, viewport_height, 0.0]);
        let center = &(horizontal / 2.0) + &(vertical / 2.0);
        let focal_vec = Vec3::new([0.0, 0.0, focal_length]);
        let lower_left_corner = &(&origin - &center) - &focal_vec;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        let x = &self.horizontal * u;
        let y = &self.vertical * v;
        let direction = self.lower_left_corner + (&(&x + &y) - &self.origin);
        Ray::new(self.origin, direction)
    }
}
