mod vector3;
mod ray;
mod sphere;
mod hit;
mod intersect;
mod directional_light;
mod png_image;

use crate::ray::Ray;
use image::{RgbImage, Rgb, ImageFormat, ImageBuffer};
use crate::vector3::Vector3;
use std::fs;
use crate::intersect::Intersect;
use crate::sphere::Sphere;

fn main() {
    let width = 256;
    let height = 256;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let sphere = Sphere::unit();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let half_width = img.width() as f64 / 2.0;
            let half_height = img.height() as f64 / 2.0;

            let u = (x as f64 - half_width) / half_width;
            let v = -(y as f64 - half_height) / half_height;

            let ray = Ray::new(
                Vector3::new(u, 5.0, v),
                Vector3::new(0.0, -1.0, 0.0),
            );

            let color = match sphere.intersect(ray) {
                None => Rgb([0, 0, 0]),
                Some(hit) => {
                    let color
                        = &(&hit.normal * 128.0) + &Vector3::new(128.0, 128.0, 128.0);
                    Rgb([color.x as u8, color.y as u8, color.z as u8])
                }
            };

            img.put_pixel(x, y, color);
        }
    }

    match std::fs::metadata("./images") {
        Err(_) => {
            std::fs::create_dir("./images");
        }
        Ok(_) => {}
    }

    img.save_with_format("./images/image.png", ImageFormat::Png);
}

