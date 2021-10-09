mod vector3;

use image::{RgbImage, Rgb, ImageFormat, ImageBuffer};
use crate::vector3::Vector3;

mod ray;
mod sphere;
mod hit;
mod intersect;
mod directionalLight;

use crate::ray::Ray;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(64, 64);

    for x in 0..32 {
        for y in 0..32 {
            img.put_pixel(x, y, Rgb([255, 0, 255]))
        }
    }

    let res = img.save_with_format("./images/image.png", ImageFormat::Png);
}

