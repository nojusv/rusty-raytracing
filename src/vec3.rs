use std::ops::{Add, Mul, MulAssign, Div, DivAssign, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: [x, y, z],
        }
    }
    pub fn length_squared(&self) -> f32 {
        self.v[0]*self.v[0] 
        + self.v[1]*self.v[1] 
        + self.v[2]*self.v[2]
    }
    pub fn length(&self) -> f32 { self.length_squared().sqrt() }
    pub fn x(&self) -> f32 { self.v[0] }
    pub fn y(&self) -> f32 { self.v[1] }
    pub fn z(&self) -> f32 { self.v[2] }
}

// vec3 + n
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

// vec3 = vec3 + vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
            ]
        }
    }
}

// vec3 *= n
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.v[0] *= t;
        self.v[1] *= t;
        self.v[2] *= t;
    }
}

// vec3 /= n
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.mul_assign(1.0 / t);
    }
}

// vec3 = vec3 * n
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] * s,
                self.v[1] * s,
                self.v[2] * s,
            ]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            v: [
                vec.v[0] * self,
                vec.v[1] * self,
                vec.v[2] * self,
            ]
        }
    }
}

// vec3 = vec3 / n
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f32) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] / s,
                self.v[1] / s,
                self.v[2] / s,
            ],
        }
    }
}

// vec3 = &vec3 / n
impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, s: f32) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] / s,
                self.v[1] / s,
                self.v[2] / s,
            ],
        }
    }
}

// -vec3
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

// vec3 = vec3 - vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

// vec3 = &Vec3 - &Vec3
impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.v[0] * v.v[0]
        + u.v[1] * v.v[1]
        + u.v[2] * v.v[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::from(
        u.v[1] * v.v[2] - u.v[2] * v.v[1],
        u.v[2] * v.v[0] - u.v[0] * v.v[2],
        u.v[0] * v.v[1] - u.v[1] * v.v[0]
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub type Point3 = Vec3;