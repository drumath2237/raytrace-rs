mod vector3;

use vector3::*;

fn main() {
    let v = Vector3::new(1, 2, 3);
    let v2 = Vector3::new(2, 3, 4);
    println!("{:?}", v + v2);
}

