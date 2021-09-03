use std::ops;

#[derive(Debug)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    pub fn new(_x: i32, _y: i32, _z: i32) -> Vector3 {
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
        assert_eq!(Vector3::new(1, 2, 3), Vector3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn add_test() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(2, 3, 4);

        assert_eq!(v1 + v2, Vector3::new(3, 5, 7));
    }

    #[test]
    fn vector_sub_test() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(1, 0, 0);

        assert_eq!(v1 - v2, Vector3::new(0, 2, 3));
    }
}


