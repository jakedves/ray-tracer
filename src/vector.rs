use cgmath::Vector3;

pub type Color = Vector3<f64>;
pub type Point = Vector3<f64>;

pub fn unit_vector(vector: &Point) -> Point {
    vector / vector_length(vector)
}

pub fn vector_length(vector: &Point) -> f64 {
    length_squared(vector).sqrt()
}

pub fn length_squared(vector: &Point) -> f64 {
    vector.x * vector.x + vector.y * vector.y + vector.z * vector.z
}
