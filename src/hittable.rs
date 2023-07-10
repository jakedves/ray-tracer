use cgmath::InnerSpace;

use crate::{
    material::Material,
    ray::Ray,
    vector::{Color, Point},
};

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Point,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point::new(0.0, 0.0, 0.0),
            normal: Point::new(0.0, 0.0, 0.0),
            material: Material::Lambertian {
                albedo: Color::new(0.0, 0.0, 0.0),
            },
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Point) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
