use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub o: Vector3,
    pub d: Vector3,
}

impl Ray {
    pub fn new(o: Vector3, d: Vector3) -> Ray {
        return Ray { o, d: Vector3::normalize(&d) };
    }

    pub fn t(&self, t: f64) -> Vector3 {
        &self.o + &(&self.d * t)
    }
}

impl std::cmp::PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d && self.o == other.o
    }
}

#[cfg(test)]
mod ray_test {
    use crate::ray::Ray;
    use crate::vector3::Vector3;

    #[test]
    fn new_test() {
        let o = Vector3::zero();
        let d = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(
            Ray::new(o, d),
            Ray {
                o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                d: Vector3::normalize(&Vector3 { x: 1.0, y: 1.0, z: 1.0 }),
            }
        )
    }

    #[test]
    fn t_test() {
        let r = Ray::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(r.t(0.1), Vector3::new(1.0, 0.1, 0.0));
    }
}