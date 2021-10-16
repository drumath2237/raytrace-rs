use std::cmp::max;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vector3::Vector3;

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let a = ray.d.length2();
        let b = 2.0 * Vector3::dot(&ray.d, &(&ray.o - &self.origin));
        let c = (&ray.o - &self.origin).length2() - self.radius * &self.radius;

        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return None;
        }

        // t1 >= t2
        let t1 = (-b + d.sqrt()) / 2.0 * a;
        let t2 = (-b - d.sqrt()) / 2.0 * a;

        // both t1,t2 is in back side of ray
        if t1.is_sign_negative() { return None; }

        let t = if t2.is_sign_positive() { t2 } else { t1 };

        let position = ray.t(t);
        let normal = &position - &self.origin;

        return Some(Hit { t, position, normal });
    }
}

impl Intersect for Scene {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let mut min_hit = Hit::new(Vector3::zero(), Vector3::zero(), f64::MAX);

        for sphere in &self.spheres {
            match sphere.intersect(ray.clone()) {
                Some(hit) => {
                    // if ray intersects sphere, update min_hit
                    if hit.t.is_sign_positive() && hit.t < min_hit.t {
                        min_hit = hit
                    }
                }
                None => {}
            }
        }

        return if min_hit.t != f64::MAX { Some(min_hit) } else { None };
    }
}

#[cfg(test)]
mod intersect_test {
    use crate::directional_light::DirectionalLight;
    use super::*;

    #[test]
    fn sphere_intersect_test() {
        let s = Sphere::unit();
        let ray = Ray::new(
            Vector3::new(-2.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );

        let result = s.intersect(ray.clone());

        assert_ne!(result, None);

        let hit = result.unwrap();

        assert_eq!(hit.position, Vector3::new(-1.0, 0.0, 0.0));
        assert_eq!(hit.normal, Vector3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn sphere_not_intersect_test() {

        // ray will not intersect simply.
        let s = Sphere::unit();
        let ray = Ray::new(
            Vector3::new(-2.0, 2.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        let result = s.intersect(ray.clone());
        assert_eq!(result, None);

        // ray will not intersect because sphere is in back side of ray
        let ray = Ray::new(
            Vector3::new(-2.0, 0.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        );
        let result = s.intersect(ray.clone());
        assert_eq!(result, None);

    }

    #[test]
    fn scene_intersect_test() {
        let light = DirectionalLight::new(Vector3::zero(), 1.0);
        let mut scene = Scene::new(light.clone());

        scene.spheres.push(
            Sphere::unit()
        );

        // ray will intersect
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., 1.));
        assert_ne!(scene.intersect(ray), None);

        // ray will not intersect because sphere is in back side
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., -1.));
        assert_eq!(scene.intersect(ray), None);

        // ray will not intersect simply.
        let ray = Ray::new(Vector3::new(2., 0., -5.), Vector3::new(0., 0., 1.));
        assert_eq!(scene.intersect(ray), None);
    }
}
