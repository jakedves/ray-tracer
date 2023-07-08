use cgmath::Vector3;

type Point = Vector3<f64>;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        return self.origin + t * self.direction;
    }
}
