use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

/// Use this type everywhere to easily change between f64 and f32
pub type FloatType = f64;

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

    pub fn print_color(&self, samples_per_pixel: usize) {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        // Divide the color by the number of samples
        let scale = 1.0 / (samples_per_pixel as FloatType);
        r *= scale;
        g *= scale;
        b *= scale;

        // Write the translated [0,255] value of each color component
        println!(
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8
        );
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

impl Add for &Vector3 {
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

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<'a> Mul<&'a Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &'a Vector3) -> Self::Output {
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

impl Mul<FloatType> for &Vector3 {
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

impl Mul<&Vector3> for FloatType {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
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
