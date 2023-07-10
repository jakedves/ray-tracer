use cgmath::InnerSpace;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vector::{near_zero, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        ray: Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = record.normal + random_unit_vector();

                if near_zero(scatter_direction) {
                    scatter_direction = record.normal;
                }

                *scattered = Ray::new(record.point, scatter_direction);
                *attenuation = *albedo;

                true
            }

            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(unit_vector(ray.direction), record.normal);

                *scattered = Ray::new(record.point, reflected + *fuzz * random_in_unit_sphere());
                *attenuation = *albedo;

                scattered.direction.dot(record.normal) > 0.0
            }
        }
    }
}
