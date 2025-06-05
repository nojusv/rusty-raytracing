mod vec3;
mod color;
use std::fs::File;
use std::io::{self, Write};
use crate::color::{Color, write_color};

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut file = File::create("output.ppm")?;

    writeln!(file, "P3\n{} {}\n255\n", image_width, image_height);

    // for j in 0..image_height {
    //     for i in 0..image_width {
    //         let pixel_color = Color::from(i as f32 /((image_width - 1) as f32), j as f32 / ((image_height - 1) as f32), 0.0);
    //         write_color(&mut file, &pixel_color);
    //     }
    // }

    Ok(())
}
