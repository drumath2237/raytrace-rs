use crate::material::Material;
use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub material: Material,
}

impl Hit {
    pub fn new(position: Vector3, normal: Vector3, t: f64, material: Material) -> Hit {
        return Hit { position, normal: Vector3::normalize(&normal), t, material };
    }
}

#[cfg(test)]
mod hit_test {
    use crate::vector3::Vector3;
    use crate::hit::Hit;
    use crate::material::Color3;
    use crate::material::Material::Specular;

    #[test]
    fn new_test() {
        let p = Vector3::new(1.0, 0.0, 0.0);
        let n = Vector3::new(2.0, 0.0, 0.0);
        let t: f64 = 1.0;
        let material = Specular;

        assert_eq!(
            Hit::new(p.clone(), n.clone(), t, material.clone()),
            Hit { position: p.clone(), normal: Vector3::normalize(&n), t, material }
        );
    }
}