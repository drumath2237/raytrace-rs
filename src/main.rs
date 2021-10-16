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
use image::hdr::rgbe8;
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

    let light = DirectionalLight::new(Vector3::new(1., -1., 1.), 1.0);

    let mut scene = Scene::new(light.clone());

    scene.spheres.push(Sphere::new(Vector3::new(0.7, 1.0, 2.), 1.0));
    scene.spheres.push(Sphere::new(Vector3::new(-1.7, 0.2, 2.), 0.2));
    scene.spheres.push(Sphere::new(Vector3::new(-0.4, 0.5, 1.6), 0.5));
    scene.spheres.push(Sphere::new(Vector3::new(1.0, 0.3, 1.), 0.3));
    scene.spheres.push(Sphere::new(Vector3::new(0., -100000.0, 0.), 100000.0));

    let camera = Camera::new(
        Vector3::new(0., 0.5, 0.),
        width as f64 / height as f64,
        60.0,
    );

    let brdf = 0.8;// std::f64::consts::PI;

    for y in 0..img.height {
        for x in 0..img.width {
            let half_width = img.width as f64 / 2.0;
            let half_height = img.height as f64 / 2.0;

            let u = (x as f64 - half_width) / half_width;
            let v = -(y as f64 - half_height) / half_height;

            let ray = camera.camera_ray(u, v);

            let color = match scene.intersect(ray.clone()) {
                None => Rgb([0, 0, 0]),
                Some(hit) => {
                    let position = ray.clone().t(hit.t);
                    let shadow_ray = Ray::new(&position + &((&hit.normal) * 0.01), &light.clone().direction * -1.0);

                    match scene.intersect(shadow_ray.clone()) {
                        Some(_) => Rgb([0, 0, 0]),
                        None => {
                            let cos_value = Vector3::dot(&shadow_ray.d, &hit.normal);
                            let radiance = cos_value * brdf * 256.0;
                            Rgb([radiance as u8, radiance as u8, radiance as u8])
                        }
                    }
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

    match img.save("./images/image.png") {
        Err(E) => {
            println!("{:?}", E)
        }
        Ok(_) => {
            println!("done.")
        }
    };
}

