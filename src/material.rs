use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::random_float,
    vec::{Color, FloatType, Vector3},
};

#[derive(Clone)]
/// Metal type with Albedo & Reflection Fuzz
pub struct Metal {
    albedo: Color,
    fuzz: FloatType,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: FloatType) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

struct Dielectric {}
impl Dielectric {
    /// Use Schlick's approximation for reflectance
    pub fn reflectance(cosine: FloatType, ref_idx: FloatType) -> FloatType {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Metal),
    // Index of Refraction
    Dielectric(FloatType),
}

impl Material {
    /// Returns attenuation color and scattered ray if found
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = hit_record.normal + Vector3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.is_near_zero() {
                    scatter_direction = hit_record.normal;
                }

                Some((*albedo, Ray::new(hit_record.point, scatter_direction)))
            }
            Material::Metal(metal) => {
                let reflected = ray_in.direction.unit_vector().reflect(hit_record.normal);
                let scattered = Ray::new(
                    hit_record.point,
                    reflected + metal.fuzz * Vector3::random_in_unit_sphere(),
                );
                if scattered.direction.dot(&hit_record.normal) > 0.0 {
                    Some((metal.albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric(index_of_refraction) => {
                let attenuation = Color::new_eq(1.0);
                let refraction_ratio = if hit_record.facing_front {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || Dielectric::reflectance(cos_theta, refraction_ratio) > random_float()
                {
                    unit_direction.reflect(hit_record.normal)
                } else {
                    unit_direction.refract(hit_record.normal, refraction_ratio)
                };

                Some((attenuation, Ray::new(hit_record.point, direction)))
            }
        }
    }
}
