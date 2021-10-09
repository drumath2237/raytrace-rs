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
    let width = 64;
    let height = 64;

    let mut img: RgbImage = ImageBuffer::new(64, 64);

    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, Rgb([
                (255 * x / width) as u8,
                (255 * y / height) as u8,
                0
            ]))
        }
    }

    let res = img.save_with_format("./images/image.png", ImageFormat::Png);
}

