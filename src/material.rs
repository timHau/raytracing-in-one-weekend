use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub(crate) trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug)]
pub(crate) struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &hit_record.normal + &Vec3::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone();
        }

        let ray = Ray::new(hit_record.point, scatter_direction);
        Some((ray, self.albedo))
    }
}

#[derive(Debug)]
pub(crate) struct Metal {
    albedo: Color,
}

impl Metal {
    pub(crate) fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_direction =
            Vec3::reflect(&ray_in.direction.as_unit_vec(), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected_direction);

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
