mod hittable;
mod ray;
mod sphere;
mod vec;

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

fn hit_sphere(center: &Point3, radius: FloatType, r: &Ray) -> FloatType {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: FloatType = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as FloatType / ASPECT_RATIO) as usize;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new_eq(0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

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
            let u = i as FloatType / (IMAGE_WIDTH - 1) as FloatType;
            let v = j as FloatType / (IMAGE_HEIGHT - 1) as FloatType;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r, &world);
            pixel_color.print_color();
        }
    }
    eprintln!("\nDone.");
}
