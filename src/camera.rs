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

#[cfg(test)]
mod camera_test {
    use crate::intersect::Intersect;
    use crate::sphere::Sphere;
    use super::*;

    #[test]
    fn new_test() {
        let position = Vector3::new(1.0, 1.0, 2.0);
        let camera = Camera::new(position.clone(), 1.0, 1.0);

        assert_eq!(camera, Camera::new(
            position.clone(),
            1.0, 1.0,
        ));
    }

    #[test]
    fn default_test() {
        let default_camera = Camera::default();
        let camera = Camera::new(Vector3::zero(), 16.0 / 9.0, 60.0);

        assert_eq!(default_camera, camera);

        let square_camera = Camera::default_square();
        let camera = Camera::new(Vector3::zero(), 1.0, 60.);

        assert_eq!(square_camera, camera);
    }

    #[test]
    fn camera_ray_test(){
        let default_camera = Camera::default();
        let small_sphere = Sphere::new(Vector3::new(0., 0., 5.), 1.0);

        let intersect_res = small_sphere.intersect(default_camera.camera_ray(-1., 1.));
        assert_eq!(intersect_res, None);

        let intersect_res = small_sphere.intersect(default_camera.camera_ray(0., 0.));
        assert_ne!(intersect_res, None);

    }
}