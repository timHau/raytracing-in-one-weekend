mod color;
mod vec3;

use color::Color;
use vec3::Vec3 as Point;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Color::new([
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            ]);
            println!("{}", color);
        }
    }

    eprintln!("Done.");
}
