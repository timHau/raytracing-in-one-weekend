use crate::{color::Color, hittable::Hittable, point::Point, vec3::Vec3};

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

    pub(crate) fn color(&self, world: &dyn Hittable, depth: u64) -> Color {
        let hit_record = world.hit(self, 0.001, f64::INFINITY);
        match hit_record {
            Some(rec) => {
                if depth <= 0 {
                    return Color::new([0.0, 0.0, 0.0]);
                }

                let target = &(&rec.point + &rec.normal) + &Vec3::random_in_hemisphere(&rec.normal);
                let next_direction = &target - &rec.point;
                let next_ray = Ray::new(rec.point, next_direction);
                next_ray.color(world, depth - 1) * 0.5
            }
            None => {
                let unit_direction = self.direction.as_unit_vec();
                let t = 0.5 * (unit_direction.y() + 1.0);
                Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
            }
        }
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
