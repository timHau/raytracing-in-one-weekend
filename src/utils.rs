use crate::{
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    point::Point,
    sphere::Sphere,
};
use rand::Rng;

pub(crate) fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub(crate) fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub(crate) fn random_scene(world: &mut HittableList) {
    let ground_material = Lambertian::new(Color::new([0.5, 0.5, 0.5]));
    let ground = Sphere::new(
        Point::new([0.0, -1000.0, 0.0]),
        1000.0,
        Box::new(ground_material),
    );
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new([
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            ]);

            let choose_mat = random_float();
            if (center - Point::new([4.0, 0.2, 0.0])).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = &Color::random(0.0, 1.0) * &Color::random(0.0, 1.0);
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
                    world.add(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
                    world.add(Box::new(sphere));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
                    world.add(Box::new(sphere));
                }
            }
        }
    }

    let material_1 = Dielectric::new(1.5);
    let sphere_1 = Sphere::new(Point::new([0.0, 1.0, 0.0]), 1.0, Box::new(material_1));
    world.add(Box::new(sphere_1));

    let material_2 = Lambertian::new(Color::new([0.4, 0.2, 0.1]));
    let sphere_2 = Sphere::new(Point::new([-4.0, 1.0, 0.0]), 1.0, Box::new(material_2));
    world.add(Box::new(sphere_2));

    let material_3 = Metal::new(Color::new([0.7, 0.6, 0.5]), 0.0);
    let sphere_3 = Sphere::new(Point::new([4.0, 1.0, 0.0]), 1.0, Box::new(material_3));
    world.add(Box::new(sphere_3));
}
