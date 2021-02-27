use crate::vec::{FloatType, Point3, Vector3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: FloatType) -> Point3 {
        self.origin + t * self.direction
    }
}
