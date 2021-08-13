use std::vec;

use crate::{point::Point, ray::Ray, vec3::Vec3};

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
pub(crate) struct HitRecord {
    pub(crate) point: Point,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(point: Point, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }

    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub(crate) struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub(crate) fn new() -> Self {
        Self { objects: vec![] }
    }

    pub(crate) fn clear(&mut self) {
        self.objects = vec![];
    }

    pub(crate) fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                None => continue,
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    temp_record = Some(hit_record);
                }
            }
        }

        temp_record
    }
}
