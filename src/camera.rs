use crate::{point::Point, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub(crate) struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn new(
        look_from: Point,
        look_at: Point,
        view_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = (&look_from - &look_at).as_unit_vec();
        let u = (view_up.cross(&w)).as_unit_vec();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * &u;
        let vertical = viewport_height * &v;
        let center = &(horizontal / 2.0) + &(vertical / 2.0);
        let lower_left_corner = &(&origin - &center) - &w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray {
        let x = &self.horizontal * s;
        let y = &self.vertical * t;
        let direction = self.lower_left_corner + (&(&x + &y) - &self.origin);
        Ray::new(self.origin, direction)
    }
}
