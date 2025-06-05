use std::fs::File;
use std::io::{self, Write};
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(file: &mut File, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)
}