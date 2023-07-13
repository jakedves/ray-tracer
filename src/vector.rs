use cgmath::{InnerSpace, Vector3};

pub type Color = Vector3<f64>;
pub type Point = Vector3<f64>;

use rand::Rng;

pub fn unit_vector(vector: Point) -> Point {
    vector / vector_length(vector)
}

pub fn vector_length(vector: Point) -> f64 {
    length_squared(vector).sqrt()
}

pub fn length_squared(vector: Point) -> f64 {
    vector.x * vector.x + vector.y * vector.y + vector.z * vector.z
}

pub fn random() -> Point {
    let mut rng = rand::thread_rng();

    Point {
        x: rng.gen_range(0.0..1.0),
        y: rng.gen_range(0.0..1.0),
        z: rng.gen_range(0.0..1.0),
    }
}

pub fn random_negative() -> Point {
    let mut rng = rand::thread_rng();

    Point {
        x: rng.gen_range(-1.0..1.0),
        y: rng.gen_range(-1.0..1.0),
        z: rng.gen_range(-1.0..1.0),
    }
}

pub fn random_in_unit_sphere() -> Point {
    loop {
        let point = random_negative();
        if length_squared(point) < 1.0 {
            return point;
        }
    }
}

pub fn random_unit_vector() -> Point {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_unit_disk() -> Point {
    let mut rng = rand::thread_rng();

    loop {
        let p = Point::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if length_squared(p) < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(vector: Point) -> bool {
    let limit = 1e-8;

    vector.x.abs() < limit && vector.y.abs() < limit && vector.z.abs() < limit
}

pub fn reflect(vector_in: Point, normal: Point) -> Point {
    vector_in - 2.0 * vector_in.dot(normal) * normal
}

pub fn hadamard(a: Point, b: Point) -> Point {
    Point::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn refract(uv: Point, normal: Point, refractive_index: f64) -> Point {
    let cos_theta = normal.dot(-uv).min(1.0);
    let r_out_perpendicular = refractive_index * (uv + cos_theta * normal);
    let r_out_parallel = -(1.0 - length_squared(r_out_perpendicular)).sqrt() * normal;

    r_out_parallel + r_out_perpendicular
}
