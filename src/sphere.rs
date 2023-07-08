use cgmath::InnerSpace;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vector::{length_squared, Point},
};

#[derive(Clone, Copy)]
struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let translation = ray.origin - self.center;
        let a = length_squared(ray.direction);
        let half_b = translation.dot(ray.direction);
        let c = length_squared(translation) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let rooted = discriminant.sqrt();

        // attempt both solutions, looking for the one in range
        let mut root = (-half_b - rooted) / a;
        if root < t_min || root > t_max {
            root = (-half_b + rooted) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        // save a hit record
        record.t = root;
        record.point = ray.at(root);
        record.normal = (record.point - self.center) / self.radius;

        return true;
    }
}
