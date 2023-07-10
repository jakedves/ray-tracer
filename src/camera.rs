use crate::{
    ray::Ray,
    vector::{unit_vector, Point},
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        view_up: Point,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(view_up.cross(w));
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin,
        }
    }
}
