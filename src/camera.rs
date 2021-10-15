use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub position: Vector3,
    pub aspect_ratio: f64,
    pub fov: f64,
}

impl Camera {
    pub fn new(position: Vector3, aspect_ratio: f64, fov: f64) -> Camera {
        return Camera { position, aspect_ratio, fov };
    }

    pub fn default() -> Camera {
        return Camera::new(Vector3::zero(), 16.0 / 9.0, 60.0);
    }

    pub fn default_square() -> Camera {
        return Camera::new(Vector3::zero(), 1.0, 60.0);
    }

    pub fn camera_ray(&self, u: f64, v: f64) -> Ray {
        let theta = self.fov * std::f64::consts::PI / 180.0;
        let f = self.aspect_ratio * (theta / 2.0).atan();

        let direction = Vector3::new(self.aspect_ratio * u, v, f);

        return Ray::new(self.position.clone(), &self.position.clone() + &direction);
    }
}