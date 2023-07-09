use cgmath::Vector3;

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
        x: rng.gen_range(-1.0..1.0),
        y: rng.gen_range(-1.0..1.0),
        z: rng.gen_range(-1.0..1.0),
    }
}

pub fn random_in_unit_sphere() -> Point {
    loop {
        let point = random();
        if length_squared(point) < 1.0 {
            return point;
        }
    }
}

pub fn random_unit_vector() -> Point {
    unit_vector(random_in_unit_sphere())
}
