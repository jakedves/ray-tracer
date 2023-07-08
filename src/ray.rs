use cgmath::{InnerSpace, Vector3};

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
        self.origin + t * self.direction
    }

    /**
     * ((x - c1)^2, (y - c2)^2, (z - c3)^2) = radius^2 is the formula for a sphere
     *
     * We are using some rearranged formula to solve a quadratic equation. We are looking
     * to see if the intersection of the ray and there sphere, has any existing real roots,
     * which happens when the discriminant of the quadratic equation is positive.
     */
    pub fn hits_sphere(&self, center: Point, radius: f64) -> bool {
        let translation = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * translation.dot(self.direction);
        let c = translation.dot(translation) - radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        discriminant > 0.0
    }
}
