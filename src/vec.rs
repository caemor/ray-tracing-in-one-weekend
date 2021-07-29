use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::utils::{random_float, random_float_range};

/// Use this type everywhere to easily change between f64 and f32
pub type FloatType = f32;
pub const PI: FloatType = core::f32::consts::PI;

/// Color Abstraction for Vector3
pub type Color = Vector3;
/// Point3 Abstraction for Vector3
pub type Point3 = Vector3;

/// 3-D Vector
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: FloatType,
    pub y: FloatType,
    pub z: FloatType,
}

impl Vector3 {
    pub fn new(x: FloatType, y: FloatType, z: FloatType) -> Self {
        Vector3 { x, y, z }
    }

    /// Create a Vector from same value (x=y=z)
    pub fn new_eq(x: FloatType) -> Self {
        Vector3 { x: x, y: x, z: x }
    }

    pub fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: FloatType, max: FloatType) -> Self {
        Self::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    /// Find a random point in a unit radius sphere

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0
        // in the same hemisphere as the normal
        {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vector3::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn length(&self) -> FloatType {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> FloatType {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> FloatType {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Returns true if the vector is close to zero in all dimensions
    pub fn is_near_zero(&self) -> bool {
        const S: FloatType = 1.0e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    /// Returns a reflected ray directon
    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        *self - 2.0 * self.dot(&normal) * normal
    }

    /// Returns a refracted ray
    pub fn refract(&self, normal: Vector3, etai_over_etat: FloatType) -> Vector3 {
        let cos_theta = -self.dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn r(&self) -> FloatType {
        self.x
    }

    pub fn g(&self) -> FloatType {
        self.y
    }

    pub fn b(&self) -> FloatType {
        self.z
    }

    pub fn print_color(&self, samples_per_pixel: usize) -> (u8, u8, u8) {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let scale = 1.0 / (samples_per_pixel as FloatType);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        // Write the translated [0,255] value of each color component
        // println!(
        //     "{} {} {}",
        //     (256.0 * r.clamp(0.0, 0.999)) as u8,
        //     (256.0 * g.clamp(0.0, 0.999)) as u8,
        //     (256.0 * b.clamp(0.0, 0.999)) as u8
        // );
        (
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        )
    }
}

impl core::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<FloatType> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: FloatType) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for FloatType {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<FloatType> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: FloatType) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<FloatType> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: FloatType) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
