use crate::hittable::{HitRecord, Hittable};

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self {
            if object.hit(ray, t_min, closest_so_far, record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        hit_anything
    }
}
