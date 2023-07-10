mod camera;
mod hittable;
mod material;
pub mod ray;
mod sphere;
mod vector;
mod world;

use std::f64::consts::PI;
use std::fs::File;
use std::io::Result;
use std::io::Write;

use material::Material;
use rand::Rng;

use camera::Camera;
use hittable::HitRecord;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vector::hadamard;
use vector::unit_vector;
use vector::Color;
use vector::Point;
use world::World;

// IMAGE
const IMAGE_WIDTH: i64 = 480;
const IMAGE_HEIGHT: i64 = 270;
const SAMPLES_PER_PIXEL: i64 = 100;
const MAX_DEPTH: i64 = 50;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

// MATERIALS
const MATERIAL_GROUND: Material = Material::Lambertian {
    albedo: Color::new(0.8, 0.8, 0.0),
};

const MATERIAL_CENTER: Material = Material::Lambertian {
    albedo: Color::new(0.1, 0.2, 0.5),
};

const MATERIAL_LEFT: Material = Material::Lambertian {
    albedo: Color::new(0.0, 0.0, 1.0),
};

const MATERIAL_RIGHT: Material = Material::Lambertian {
    albedo: Color::new(1.0, 0.0, 0.0),
};

// FILE
const MAX_COLOR: i64 = 255;
const FILE_TYPE: &str = "P3";

fn ray_color(ray: Ray, world: &World, depth: i64) -> Color {
    let mut record = HitRecord::new();

    if depth <= 0 {
        // println!("{}", "(0, 0, 0) as depth less than or equal to 0");
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut record) {
        // will be updated, using defaults
        let mut scattered = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        let material = record.material;
        let ray_does_scatter = material.scatter(ray, &record, &mut attenuation, &mut scattered);

        if ray_does_scatter {
            // println!("{}", "computing Hadamard as ray scattered");
            return hadamard(attenuation, ray_color(scattered, world, depth - 1));
        }

        // println!("{}", "(0, 0, 0) as ray did not scatter");
        return Color::new(0.0, 0.0, 0.0);
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

fn write_color(file: &mut File, color: Color, samples: i64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let string = format!(
        "{} {} {}\n",
        (r.clamp(0.0, 0.999) * 256.0) as i64,
        (g.clamp(0.0, 0.999) * 256.0) as i64,
        (b.clamp(0.0, 0.999) * 256.0) as i64,
    );

    file.write_all(string.as_bytes()).unwrap();
}

fn main() {
    let r = (PI / 4.0).cos();

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(-r, 0.0, -1.0), r, MATERIAL_LEFT)),
        Box::new(Sphere::new(Point::new(r, 0.0, -1.0), r, MATERIAL_RIGHT)),
    ];

    let camera = Camera::new(90.0, ASPECT_RATIO);

    // render
    let header = format!(
        "{}\n{} {}\n{}\n",
        FILE_TYPE, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR
    );

    // code can panic here
    let mut file = write_file(header.as_str()).unwrap();
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let r = camera.get_ray(u, v);

                color += ray_color(r, &world, MAX_DEPTH);
                // COLOR = (0, 0, 0) here
                // maybe ray_color always returns (0, 0, 0)
            }

            write_color(&mut file, color, SAMPLES_PER_PIXEL);
        }
    }
}
