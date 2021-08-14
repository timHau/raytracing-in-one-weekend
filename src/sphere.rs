use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    point::Point,
    ray::Ray,
};

pub(crate) struct Sphere<M>
where
    M: Material + Copy + 'static,
{
    center: Point,
    radius: f64,
    material: Box<M>,
}

impl<M> Sphere<M>
where
    M: Material + Copy + 'static,
{
    pub(crate) fn new(center: Point, radius: f64, material: Box<M>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M> Hittable for Sphere<M>
where
    M: Material + Copy + 'static,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_square();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (&point - &self.center) / self.radius;
        let mut hit_record = HitRecord::new(point, normal, root, false, Box::new(*self.material));

        let outward_normal = (&hit_record.point - &self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
