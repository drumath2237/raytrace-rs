mod vector3;
mod ray;
mod sphere;
mod hit;
mod intersect;
mod directional_light;
mod png_image;
mod camera;
mod scene;

use crate::ray::Ray;
use image::{RgbImage, Rgb, ImageFormat, ImageBuffer};
use crate::vector3::Vector3;
use std::fs;
use crate::camera::Camera;
use crate::directional_light::DirectionalLight;
use crate::intersect::Intersect;
use crate::png_image::PngImage;
use crate::scene::Scene;
use crate::sphere::Sphere;

fn main() {
    let width = 960;
    let height = 540;

    let mut img = PngImage::new(width, height);

    let light = DirectionalLight::new(Vector3::zero(), 1.0);

    let mut scene = Scene::new(light.clone());

    scene.spheres.push(Sphere::new(
        Vector3::new(0.7, 0., 2.), 1.0,
    ));

    scene.spheres.push(Sphere::new(
        Vector3::new(-0.7, 0., 2.), 1.0,
    ));

    scene.spheres.push(Sphere::new(
        Vector3::new(0., -100000.0 - 1.0, 2.0), 100000.0,
    ));

    let camera = Camera::default();

    for y in 0..img.height {
        for x in 0..img.width {
            let half_width = img.width as f64 / 2.0;
            let half_height = img.height as f64 / 2.0;

            let u = (x as f64 - half_width) / half_width;
            let v = -(y as f64 - half_height) / half_height;

            let ray = camera.camera_ray(u, v);

            let color = match scene.intersect(ray) {
                None => Rgb([0, 0, 0]),
                Some(hit) => {
                    let color
                        = &(&hit.normal * 128.0) + &Vector3::new(128.0, 128.0, 128.0);
                    Rgb([color.x as u8, color.y as u8, color.z as u8])
                }
            };

            img.set_pixel(x, y, color);
        }
    }

    match std::fs::metadata("./images") {
        Err(_) => {
            std::fs::create_dir("./images");
        }
        Ok(_) => {}
    }

     match img.save("./images/image.png"){
         Err(E)=>{
             println!("{:?}", E)
         },
         Ok(_)=>{
             println!("done.")
         }
     };
}

