use std::ops::{Add, Mul, Div, Neg};

pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: [x, y, z],
        }
    }
    pub fn x(&self) -> f32 {
        self.v[0]
    }
    pub fn y(&self) -> f32 {
        self.v[1]
    }
    pub fn z(&self) -> f32 {
        self.v[2]
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, s: f32) -> Self {
        Self {
            v: [
                self.v[0] + s,
                self.v[1] + s,
                self.v[2] + s,
            ]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Self {
            v: [
                self.v[0] * s,
                self.v[1] * s,
                self.v[2] * s,
            ]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        Self {
            v: [
                self.v[0] / s,
                self.v[1] / s,
                self.v[2] / s,
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            v: [
                -self.v[0],
                -self.v[1],
                -self.v[2],
            ],
        }
    }
}