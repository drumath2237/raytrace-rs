mod vector3;

use vector3::*;

fn main() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(2.0, 3.0, 4.0);
    let d = v1 + v2;
    let d = Vector3::dot(&v1, &v2);
    println!("{:?}", v1);
}

