mod vec3;
mod color;
mod ray;
use std::fs::File;
use std::io::{self, Write};
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, unit_vector};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = {
        let h = (image_width as f32 / aspect_ratio) as i32;
        if h < 1 { 1 } else { h }
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f32) / image_height as f32);
    let camera_center = Point3::from(0.0, 0.0, 0.0);

    // Vector from camera to top left of viewport
    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    // Vector from camera also to top left of viewport, since
    // the bottom of the viewport goes down, have to use negative y
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    // Calculate the vectors for distances between the pixels
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Vector to the top left pixel
    let viewport_upper_left = camera_center -
                            - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    
    // Move by half a pixel width to capture the center
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("output.ppm")?;

    // Required headers?
    writeln!(file, "P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut file, &pixel_color);
        }
    }

    Ok(())
}
