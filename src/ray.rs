use crate::vec3::{ Vec3, Point3 };

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn from(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}