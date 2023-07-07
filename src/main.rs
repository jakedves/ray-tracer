use std::fs::File;
use std::io::Result;
use std::io::Write;

// 1920 x 1080 but 4x smaller dimensions
const IMAGE_WIDTH: i64 = 480;
const IMAGE_HEIGHT: i64 = 270;
const MAX_COLOR: i64 = 255;
const FILE_TYPE: &str = "P3";

fn write_file(content: &str) -> Result<()> {
    let mut file = File::create("render.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let mut content = "".to_string();

    let header = format!(
        "{}\n{} {}\n{}\n",
        FILE_TYPE, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR
    )
    .to_string();

    content += &header;

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64 * 255.999;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.999;
            let b = 0.25 * 255.999;

            content += &format!("{} {} {}\n", r as i64, g as i64, b as i64).to_string();
        }
    }

    // code can panic here
    write_file(content.as_str()).unwrap();
}
