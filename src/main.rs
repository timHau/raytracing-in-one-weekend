mod camera;
mod color;
mod hittable;
mod material;
mod point;
mod ray;
mod sphere;
mod utils;
mod vec3;

use point::Point;

use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Lambertian, Metal},
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
    let mut world = HittableList::new();

    let material_center = Lambertian::new(color![0.7, 0.3, 0.3]);
    let sphere_center = Sphere::new(point!(0.0, 0.0, -1.0), 0.5, &material_center);
    world.add(&sphere_center);

    let material_left = Metal::new(color![0.8, 0.8, 0.8], 0.3);
    let sphere_left = Sphere::new(point!(-1.0, 0.0, -1.0), 0.5, &material_left);
    world.add(&sphere_left);

    let material_right = Metal::new(color!(0.8, 0.6, 0.2), 1.0);
    let sphere_right = Sphere::new(point!(1.0, 0.0, -1.0), 0.5, &material_right);
    world.add(&sphere_right);

    let material_world = Lambertian::new(color![0.8, 0.8, 0.0]);
    let sphere_world = Sphere::new(point!(0.0, -100.5, -1.0), 100.0, &material_world);
    world.add(&sphere_world);

    // Camera
    let camera = Camera::new();

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
