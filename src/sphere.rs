use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
}

impl Sphere {
    fn new(origin: Vector3, radius: f64) -> Sphere {
        return Sphere { origin, radius };
    }
}

#[cfg(test)]
mod sphere_test {
    use crate::sphere::Sphere;
    use crate::vector3::Vector3;

    #[test]
    fn new_test() {
        assert_eq!(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0), Sphere { origin: Vector3::zero(), radius: 1.0 });
    }
}