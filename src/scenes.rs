use crate::{
    hittable::HittableList,
    material::{Material, Metal},
    sphere::Sphere,
    utils::random_float,
    vec::{Color, FloatType, Point3, PI},
};

#[allow(unused)]
pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..=11 {
        let a = a as FloatType;
        for b in -11..=11 {
            let b = b as FloatType;
            let choose_mat = random_float();
            let center = Point3::new(a + 0.9 * random_float(), 0.2, b + 0.9 * random_float());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                //let sphere_material;

                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Lambertian(albedo),
                        )));
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_float();
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Metal(Metal::new(albedo, fuzz)),
                        )));
                    }
                    _ => {
                        // Glass
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Dielectric(1.5),
                        )))
                    }
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

#[allow(unused)]
pub fn three_balls_scene() -> HittableList {
    let r = (PI / 4.0).cos();
    let mut world = HittableList::default();

    let material_ground = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    //let material_center = Material::Dielectric(1.5);
    let material_left = Material::Dielectric(1.5);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.50,
        material_right,
    )));
    world
}
