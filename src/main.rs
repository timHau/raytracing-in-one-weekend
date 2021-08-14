mod camera;
mod color;
mod hittable;
mod material;
mod point;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::f64::consts::PI;

use point::Point;

use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u64;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let material_center = Lambertian::new(color!(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(color!(0.8, 0.6, 0.2), 0.0);
    let material_world = Lambertian::new(color![0.8, 0.8, 0.0]);

    let sphere_center = Sphere::new(point!(0.0, 0.0, -1.0), 0.5, &material_center);
    let sphere_left = Sphere::new(point!(-1.0, 0.0, -1.0), 0.5, &material_left);
    let sphere_right = Sphere::new(point!(1.0, 0.0, -1.0), 0.5, &material_right);
    let sphere_world = Sphere::new(point!(0.0, -100.5, -1.0), 100.0, &material_world);

    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_right);
    world.add(&sphere_world);

    // Camera
    let look_from = point!(3.0, 3.0, 2.0);
    let look_at = point!(0.0, 0.0, -1.0);
    let view_up = vec3!(0.0, 1.0, 0.0);
    let vertical_fov = 20.0;
    let dist_to_focus = (look_from - look_at).len();
    let aperture = 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        vertical_fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = color!(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + utils::random_float()) / ((image_width - 1) as f64);
                let v = (j as f64 + utils::random_float()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray.color(&world, max_depth);
            }

            println!("{}", pixel_color.write(samples_per_pixel));
        }
    }

    eprintln!("Done.");
}
