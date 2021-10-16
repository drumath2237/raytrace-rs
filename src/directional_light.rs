use crate::vector3::Vector3;

#[derive(Debug, PartialEq, Clone)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vector3, intensity: f64) -> DirectionalLight {
        return DirectionalLight {
            direction:Vector3::normalize(&direction),
            intensity,
        };
    }

    pub fn new_one(direction: Vector3) -> DirectionalLight {
        return DirectionalLight::new(direction, 1.0);
    }
}

#[cfg(test)]
mod directional_light_test {
    use crate::directional_light::DirectionalLight;
    use crate::vector3::Vector3;

    #[test]
    fn new_test() {
        let d = Vector3::new(1.0, 0.0, 0.0);
        let i = 2.0;

        assert_eq!(
            DirectionalLight::new(d.clone(), i),
            DirectionalLight {
                direction: Vector3::normalize(&d),
                intensity: i,
            }
        )
    }

    #[test]
    fn new_one_test() {
        let d = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(
            DirectionalLight::new_one(d.clone()),
            DirectionalLight {
                direction: Vector3::normalize(&d),
                intensity: 1.0,
            }
        )
    }
}