use crate::{ray::Ray, vector::Point};

pub trait Hittable {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Point,
    pub t: f64,
}

impl HitRecord {
    pub fn new(point: Point, normal: Point, t: f64) -> Self {
        HitRecord { point, normal, t }
    }
}
