use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        return Vector3 { x, y, z };
    }

    pub fn zero() -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    pub fn normalize(v: &Vector3) -> Vector3 {
        return v/v.length();
    }

    pub fn length(&self) -> f64 {
        return self.length2().sqrt();
    }

    pub fn length2(&self) -> f64 {
        return Vector3::dot(&self, &self);
    }

    pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
        return v1.x * v2.x +
            v1.y * v2.y +
            v1.z * v2.z;
    }

    pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
        return Vector3::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        );
    }
}

impl ops::Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        return Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
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

        assert_eq!(&v1 + &v2, Vector3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn vector_sub_test() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(&v1 - &v2, Vector3::new(0.0, 2.0, 3.0));
    }

    #[test]
    fn zero_test() {
        assert_eq!(Vector3::zero(), Vector3 { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn length_test() {
        let vec = Vector3::zero();
        assert_eq!(vec.length(), 0.0);

        let vec = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(vec.length(), (3.0 as f64).sqrt());
        assert_eq!(vec.length(), vec.length2().sqrt());
    }

    #[test]
    fn length2_test() {
        let vec = Vector3::zero();
        assert_eq!(vec.length2(), 0.0);

        let vec = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(vec.length2(), (3.0 as f64));
    }

    #[test]
    fn normalize_test() {
        let vec = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(Vector3::normalize(&vec),
                   Vector3::new(1.0 / (3.0 as f64).sqrt(), 1.0 / (3.0 as f64).sqrt(), 1.0 / (3.0 as f64).sqrt()))
    }

    #[test]
    fn dot_test() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(Vector3::dot(&v1, &v2), 2.0 + 6.0 + 12.0);

        assert_eq!(Vector3::dot(&Vector3::zero(), &Vector3::zero()), 0.0);
    }

    #[test]
    fn cross_test() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        assert_eq!(Vector3::cross(&v1, &v2), Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn mul_test() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(&v * 3.0, Vector3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn div_test() {
        let v = Vector3::new(3.0, 6.0, 9.0);
        assert_eq!(&v / 3.0, Vector3::new(1.0, 2.0, 3.0));
    }
}


