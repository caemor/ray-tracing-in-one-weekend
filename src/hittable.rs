use std::ops::RangeInclusive;

use crate::{
    material::Material,
    ray::Ray,
    vec::{FloatType, Point3, Vector3},
};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vector3,
    pub material: &'a Material,
    pub t: FloatType,
    pub facing_front: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new_face_normal(
        point: Point3,
        t: FloatType,
        r: &Ray,
        outward_normal: Vector3,
        material: &'a Material,
    ) -> HitRecord<'a> {
        let facing_front = r.direction.dot(&outward_normal) < 0.0;
        HitRecord {
            point,
            t,
            facing_front,
            normal: match facing_front {
                true => outward_normal,
                false => -outward_normal,
            },
            material,
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: FloatType, t_max: FloatType) -> Option<HitRecord> {
        let mut anything_hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                anything_hit = Some(hit);
            }
        }

        anything_hit
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &crate::ray::Ray, t_min: FloatType, t_max: FloatType) -> Option<HitRecord>;
}
