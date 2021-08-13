mod color;
mod hittable;
mod point;
mod ray;
mod sphere;
mod vec3;

use point::Point;
use ray::Ray;

use crate::{hittable::HittableList, sphere::Sphere};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u64;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;

    // World
    let mut world = HittableList::new();
    let sphere_1 = Sphere::new(point!(0.0, 0.0, -1.0), 0.5);
    world.add(&sphere_1);
    let sphere_2 = Sphere::new(point!(0.0, -100.5, -1.0), 100.0);
    world.add(&sphere_2);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point!(0.0, 0.0, 0.0);
    let horizontal = vec3!(viewport_width, 0.0, 0.0);
    let vertical = vec3!(0.0, viewport_height, 0.0);
    let center = &(horizontal / 2.0) + &(vertical / 2.0);
    let focal_vec = vec3!(0.0, 0.0, focal_length);
    let lower_left_corner = &(&origin - &center) - &focal_vec;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / ((image_width - 1) as f64);
            let v = j as f64 / ((image_height - 1) as f64);
            let x = &horizontal * u;
            let y = &vertical * v;
            let direction = lower_left_corner + (&(&x + &y) - &origin);
            let ray = Ray::new(origin, direction);
            println!("{}", ray.color(&world));
        }
    }

    println!("center: {}, lower: {}", center, lower_left_corner);
    eprintln!("Done.");
}