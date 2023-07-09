mod hittable;
pub mod ray;
mod sphere;
mod vector;
mod world;

use std::fs::File;
use std::io::Result;
use std::io::Write;

use hittable::HitRecord;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vector::unit_vector;
use vector::Color;
use vector::Point;
use world::World;

// IMAGE
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i64 = 480;
const IMAGE_HEIGHT: i64 = 270;

// CAMERA
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const ORIGIN: Point = Point::new(0.0, 0.0, 0.0);
const HORIZONTAL: Point = Point::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: Point = Point::new(0.0, VIEWPORT_HEIGHT, 0.0);

// FILE
const MAX_COLOR: i64 = 255;
const FILE_TYPE: &str = "P3";

fn ray_color(ray: Ray, world: &World) -> Color {
    let mut record = HitRecord::new();
    if world.hit(ray, 0.0, f64::INFINITY, &mut record) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_file(content: &str) -> Result<File> {
    let mut file = File::create("render.ppm")?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

fn write_color(file: &mut File, color: Color) {
    let string = format!(
        "{} {} {}\n",
        (color.x * 255.999) as i64,
        (color.y * 255.999) as i64,
        (color.z * 255.999) as i64
    );
    file.write_all(string.as_bytes()).unwrap();
}

fn main() {
    // Setup camera: (0, 0, 0) (center), halfway down the max height,
    // halfway left from max width, and moved backwards by focal length
    let lower_left_corner: Point =
        ORIGIN - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Point::new(0.0, 0.0, FOCAL_LENGTH);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // render
    let header = format!(
        "{}\n{} {}\n{}\n",
        FILE_TYPE, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR
    );

    // code can panic here
    let mut file = write_file(header.as_str()).unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                ORIGIN,
                lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );

            let color = ray_color(r, &world);

            write_color(&mut file, color);
        }
    }
}
