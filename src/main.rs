use std::fs::File;
use std::io::Result;
use std::io::Write;

use cgmath::Vector3;

// 1920 x 1080 but 4x smaller dimensions
const IMAGE_WIDTH: i64 = 480;
const IMAGE_HEIGHT: i64 = 270;
const MAX_COLOR: i64 = 255;
const FILE_TYPE: &str = "P3";

type Color = Vector3<f64>;
type Point = Vector3<f64>;

fn write_file(content: &str) -> Result<File> {
    let mut file = File::create("render.ppm")?;
    file.write_all(content.as_bytes())?;
    return Ok(file);
}

fn write_color(file: &mut File, color: Color) {
    let string = format!("{} {} {}\n", color.x as i64, color.y as i64, color.z as i64);
    file.write(&string.as_bytes()).unwrap();
}

fn main() {
    let header = format!(
        "{}\n{} {}\n{}\n",
        FILE_TYPE, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR
    )
    .to_string();

    // code can panic here
    let mut file = write_file(header.as_str()).unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64 * 255.999,
                j as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.999,
                0.25 * 255.999,
            );

            write_color(&mut file, color);
        }
    }
}
