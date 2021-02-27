mod ray;
mod vec;

use ray::Ray;
use vec::*;

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new_eq(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: FloatType, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

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

            let pixel_color = ray_color(&r);
            pixel_color.print_color();
        }
    }
    eprintln!("\nDone.");
}
