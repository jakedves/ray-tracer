use cgmath::InnerSpace;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vector::{
        near_zero, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color,
    },
};

use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
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

            Material::Dielectric { refraction_index } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if record.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = unit_vector(ray.direction);
                let cos_theta = record.normal.dot(-unit_direction).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let mut rng = rand::thread_rng();

                let direction =
                    if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
                        reflect(unit_direction, record.normal)
                    } else {
                        refract(unit_direction, record.normal, refraction_ratio)
                    };

                *scattered = Ray::new(record.point, direction);

                true
            }
        }
    }
}

pub fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
