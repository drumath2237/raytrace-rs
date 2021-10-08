use crate::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,
}

impl DirectionalLight {
    pub fn new(position: Vector3, direction: Vector3, intensity: f64) -> DirectionalLight {
        return DirectionalLight {
            direction,
            intensity,
        };
    }

    pub fn new_one(position: Vector3, direction: Vector3) -> DirectionalLight {
        return DirectionalLight {
            direction,
            intensity: 1.0,
        };
    }
}

#[cfg(test)]
mod directional_light_test {
    use crate::directionalLight::DirectionalLight;
    use crate::vector3::Vector3;

    #[test]
    fn new_test() {
        let p = Vector3::new(1.0, 2.0, 3.0);
        let d = Vector3::new(1.0, 0.0, 0.0);
        let i = 2.0;

        assert_eq!(
            DirectionalLight::new(p.clone(), d.clone(), i),
            DirectionalLight {
                direction: d,
                intensity: i,
            }
        )
    }

    #[test]
    fn new_one_test() {
        let p = Vector3::new(1.0, 2.0, 3.0);
        let d = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(
            DirectionalLight::new_one(p.clone(), d.clone()),
            DirectionalLight {
                direction: d,
                intensity: 1.0,
            }
        )
    }
}