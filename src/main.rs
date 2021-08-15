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

use crate::{camera::Camera, hittable::HittableList};

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200 as u64;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    utils::random_scene(&mut world);

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);
    let view_up = vec3!(0.0, 1.0, 0.0);
    let vertical_fov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
            for _s in 0..samples_per_pixel {
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
