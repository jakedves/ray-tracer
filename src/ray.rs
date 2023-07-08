use cgmath::{InnerSpace, Vector3};

use crate::vector::length_squared;

type Point = Vector3<f64>;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    /**
     * ((x - c1)^2, (y - c2)^2, (z - c3)^2) = radius^2 is the formula for a sphere
     *
     * We are using some rearranged formula to solve a quadratic equation. We are looking
     * to see if the intersection of the ray and there sphere, has any existing real roots,
     * which happens when the discriminant of the quadratic equation is positive.
     *
     * This gives us the parameter t, at which our ray: origin + t*dir = point on sphere
     *
     * We only get one value of t, as we only need to render the first point we see with
     * our ray.
     */
    pub fn hits_sphere(self, center: Point, radius: f64) -> f64 {
        let translation = self.origin - center;
        let a = length_squared(self.direction);
        let half_b = translation.dot(self.direction);
        let c = length_squared(translation) - radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
