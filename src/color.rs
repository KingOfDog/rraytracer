use std::ops::{Add, AddAssign, Mul, MulAssign};

use glam::Vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f32 {
        self.0.x
    }
    pub fn g(&self) -> f32 {
        self.0.y
    }
    pub fn b(&self) -> f32 {
        self.0.z
    }

    pub fn to_rgb_bytes(&self) -> [u8; 3] {
        let r = (self.r().clamp(0., 1.) * 255.) as u8;
        let g = (self.g().clamp(0., 1.) * 255.) as u8;
        let b = (self.b().clamp(0., 1.) * 255.) as u8;
        [r, g, b]
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Color(Vec3::new(value[0], value[1], value[2]))
    }
}
