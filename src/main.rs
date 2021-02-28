mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec;

use camera::{Camera, ASPECT_RATIO};
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec::*;

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(hit) = world.hit(r, 0.0..=FloatType::INFINITY) {
        return 0.5 * (hit.normal + Color::new_eq(1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new_eq(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_float() -> FloatType {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn main() {
    // Image
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as FloatType / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 400;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render

    // Colors in Ascii
    println!("P3");
    // Columns and Rows
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    // 255 for Max Color
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new_eq(0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as FloatType + random_float()) / (IMAGE_WIDTH - 1) as FloatType;
                let v = (j as FloatType + random_float()) / (IMAGE_HEIGHT - 1) as FloatType;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            pixel_color.print_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}
