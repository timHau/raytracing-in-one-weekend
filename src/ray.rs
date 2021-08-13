use crate::{color::Color, point::Point, vec3::Vec3};

#[derive(Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub(crate) fn at(&self, t: f64) -> Point {
        self.origin + &self.direction * t
    }

    pub(crate) fn hit_sphere(&self, center: &Point, radius: f64) -> bool {
        let oc = &self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = oc.dot(&self.direction) * 2.0;
        let c = oc.dot(&oc) - radius * radius;
        b * b - 4.0 * a * c > 0.0
    }

    pub(crate) fn color(&self) -> Color {
        if self.hit_sphere(&Point::new([0.0, 0.0, -1.0]), 0.5) {
            return Color::new([1.0, 0.0, 0.0]);
        }
        let unit_direction = self.direction.as_unit_vec();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::{point::Point, vec3::Vec3};

    #[test]
    fn at() {
        let origin = Point::new([0.0, 0.0, 0.0]);
        let direction = Vec3::new([1.0, 1.0, 0.0]);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.at(0.5), Point::new([0.5, 0.5, 0.0]));
    }
}
