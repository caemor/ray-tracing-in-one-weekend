mod camera;
mod hittable;
mod material;
mod ray;
mod scenes;
mod sphere;
mod utils;
mod vec;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use scenes::*;
use utils::*;
use vec::*;

use rayon::prelude::*;

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    // If we've exceeded the ray bounce limit no more light is gathered
    if depth == 0 {
        return Color::new_eq(0.0);
    }

    if let Some(hit_record) = world.hit(r, 0.001, FloatType::MAX) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new_eq(0.0)
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new_eq(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    //pub const ASPECT_RATIO: FloatType = 16.0 / 9.0;
    const ASPECT_RATIO: FloatType = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as FloatType / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 10;
    const MAX_DEPTH: usize = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; //(look_from - look_at).length();
    let aperture = 0.1; //2.0;
    let cam = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    // Colors in Ascii
    println!("P3");
    // Columns and Rows
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    // 255 for Max Color
    println!("255");

    let image = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            eprint!("\rScanlines remaining: {} ", j);
            (0..IMAGE_WIDTH)
                .into_iter()
                .map(|i| {
                    let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let u =
                                (i as FloatType + random_float()) / (IMAGE_WIDTH - 1) as FloatType;
                            let v =
                                (j as FloatType + random_float()) / (IMAGE_HEIGHT - 1) as FloatType;
                            let r = cam.get_ray(u, v);
                            ray_color(&r, &world, MAX_DEPTH)
                        })
                        //.reduce(|| Color::new_eq(0.0), |a, b| a + b);
                        .fold(Color::new_eq(0.0), |acc, b| acc + b);

                    pixel_color.print_color(SAMPLES_PER_PIXEL)
                })
                .collect::<Vec<(u8, u8, u8)>>()
        })
        .collect::<Vec<(u8, u8, u8)>>();
    for pixel in image {
        println!("{} {} {}", pixel.0, pixel.1, pixel.2);
    }
    eprintln!("\nDone.");
}
