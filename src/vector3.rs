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
