use crate::ray::Ray;
use crate::hit::Hit;
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

        let t1 = (-b + d.sqrt()) / 2.0 * a;
        let t2 = (-b - d.sqrt()) / 2.0 * a;

        let t = if t1 <= t2 { t1 } else { t2 };

        let position = ray.t(t);
        let normal = &position - &self.origin;

        return Some(Hit { t, position, normal });
    }
}

#[cfg(test)]
mod intersect_test {
    use crate::intersect::Intersect;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vector3::Vector3;

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
        let s = Sphere::unit();
        let ray = Ray::new(
            Vector3::new(-2.0, 2.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );

        let result = s.intersect(ray.clone());

        assert_eq!(result, None);
    }
}
