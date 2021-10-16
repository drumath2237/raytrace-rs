use crate::material::{Color3, Material};
use crate::material::Material::Diffuse;
use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(origin: Vector3, radius: f64, material: Material) -> Sphere {
        return Sphere { origin, radius, material };
    }

    pub fn unit() -> Sphere {
        return Sphere::new(
            Vector3::zero(), 1.0,
            Diffuse(Color3::new(1., 1., 1.)),
        );
    }
}

#[cfg(test)]
mod sphere_test {
    use crate::material::Color3;
    use crate::material::Material::Diffuse;
    use crate::sphere::Sphere;
    use crate::vector3::Vector3;

    #[test]
    fn new_test() {
        assert_eq!(
            Sphere::new(
                Vector3::new(0.0, 0.0, 0.0), 1.0,
                Diffuse(Color3::zero()),
            ),
            Sphere { origin: Vector3::zero(), radius: 1.0, material: Diffuse(Color3::zero()) }
        );
    }

    #[test]
    fn unit_test() {
        assert_eq!(
            Sphere::unit(),
            Sphere {
                origin: Vector3::zero(),
                radius: 1.0,
                material: Diffuse(Color3::new(1., 1., 1.)),
            }
        );
    }
}