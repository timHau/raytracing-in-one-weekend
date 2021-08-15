use crate::{point::Point, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub(crate) fn new(
        look_from: Point,
        look_at: Point,
        view_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).as_unit_vec();
        let u = (view_up.cross(&w)).as_unit_vec();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let center = horizontal / 2.0 + vertical / 2.0;
        let lower_left_corner = origin - center - focus_dist * &w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_direction = self.lens_radius * &Vec3::random_in_unit_sphere();
        let offset = self.u * ray_direction.x() + self.v * ray_direction.y();
        let h = s * &self.horizontal;
        let v = t * &self.vertical;
        let direction = self.lower_left_corner + h + v - self.origin - offset;
        Ray::new(self.origin + offset, direction)
    }
}
