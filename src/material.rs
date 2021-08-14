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
    fuzz: f64,
}

impl Metal {
    pub(crate) fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_direction =
            Vec3::reflect(&ray_in.direction.as_unit_vec(), &hit_record.normal);
        let fuzzy_direction = reflected_direction + self.fuzz * &Vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, fuzzy_direction);

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub(crate) fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.as_unit_vec();
        let refracted = Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio);

        let scattered = Ray::new(hit_record.point, refracted);
        Some((scattered, Color::new([1.0, 1.0, 1.0])))
    }
}
