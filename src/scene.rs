use crate::directional_light::DirectionalLight;
use crate::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct Scene {
    pub light: DirectionalLight,
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(light: DirectionalLight) -> Scene {
        return Scene {
            spheres: vec![],
            light,
        };
    }
}

#[cfg(test)]
mod scene_test {
    use crate::vector3::Vector3;
    use super::*;

    #[test]
    fn new_test() {
        let light = DirectionalLight::new(
            Vector3::new(0., -1., 0.),
            1.0,
        );
        let scene = Scene::new(light.clone());

        assert_eq!(
            scene, Scene {
                spheres: vec![],
                light
            }
        )
    }
}