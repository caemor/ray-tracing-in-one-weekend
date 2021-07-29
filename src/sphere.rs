use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec::{FloatType, Point3},
};

pub struct Sphere {
    center: Point3,
    radius: FloatType,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: FloatType, material: Material) -> Sphere {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: FloatType, t_max: FloatType) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min && t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min && t_max < root {
                return None;
            }
        }

        let t = root;
        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new_face_normal(
            point,
            t,
            r,
            outward_normal,
            &self.material,
        ))
    }
}
