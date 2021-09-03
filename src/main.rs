mod vector3;

use vector3::*;

fn main() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(2.0, 3.0, 4.0);
    println!("{:?}", v + v2);
}

