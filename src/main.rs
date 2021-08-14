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
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};

fn random_scene(world: &mut HittableList) {
    let ground_material = Lambertian::new(color![0.5, 0.5, 0.5]);
    let ground = Sphere::new(point![0.0, -1000.0, 0.0], 1000.0, Box::new(ground_material));
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let center = point![
                a as f64 + 0.9 * utils::random_float(),
                0.2,
                b as f64 + 0.9 * utils::random_float()
            ];

            let choose_mat = utils::random_float();
            if (center - point![4.0, 0.2, 0.0]).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = &Color::random(0.0, 1.0) * &Color::random(0.0, 1.0);
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
                    world.add(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = utils::random_range(0.0, 0.5);
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
    let sphere_1 = Sphere::new(point![0.0, 1.0, 0.0], 1.0, Box::new(material_1));
    world.add(Box::new(sphere_1));

    let material_2 = Lambertian::new(color![0.4, 0.2, 0.1]);
    let sphere_2 = Sphere::new(point![-4.0, 1.0, 0.0], 1.0, Box::new(material_2));
    world.add(Box::new(sphere_2));

    let material_3 = Metal::new(color![0.4, 0.2, 0.1], 0.0);
    let sphere_3 = Sphere::new(point![4.0, 1.0, 0.0], 1.0, Box::new(material_3));
    world.add(Box::new(sphere_3));
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200 as u64;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    random_scene(&mut world);

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);
    let view_up = vec3!(0.0, 1.0, 0.0);
    let vertical_fov = 10.0;
    let dist_to_focus = (look_from - look_at).len();
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
