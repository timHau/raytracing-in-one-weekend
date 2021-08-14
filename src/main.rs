mod camera;
mod color;
mod hittable;
mod point;
mod ray;
mod sphere;
mod utils;
mod vec3;

use point::Point;

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u64;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    let sphere_1 = Sphere::new(point!(0.0, 0.0, -1.0), 0.5);
    world.add(&sphere_1);
    let sphere_2 = Sphere::new(point!(0.0, -100.5, -1.0), 100.0);
    world.add(&sphere_2);

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
