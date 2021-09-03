use std::ops;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Vector3 {
        return Vector3 {
            x: _x,
            y: _y,
            z: _z,
        };
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        return Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl ops::Sub<Vector3> for Vector3{
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        todo!()
    }
}

impl std::cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        return
            self.x == other.x
                && self.y == other.y
                && self.z == other.z;
    }
}

#[cfg(test)]
mod vector_test {
    use crate::vector3::Vector3;

    #[test]
    fn vector_new_test() {
        assert_eq!(Vector3::new(1.0, 2.0, 3.0), Vector3 { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn add_test() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(v1 + v2, Vector3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn vector_sub_test() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(v1 - v2, Vector3::new(0.0, 2.0, 3.0));
    }
}


