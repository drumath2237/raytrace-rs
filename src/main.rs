mod vector3;
mod ray;
mod sphere;
mod hit;
mod intersect;
mod directional_light;
mod png_image;
mod camera;
mod scene;
mod material;

use crate::ray::Ray;
use image::{RgbImage, Rgb, ImageFormat, ImageBuffer};
use crate::vector3::Vector3;
use std::fs;
use crate::camera::Camera;
use crate::directional_light::DirectionalLight;
use crate::intersect::Intersect;
use crate::material::Color3;
use crate::material::Material::{Diffuse, Specular};
use crate::png_image::PngImage;
use crate::scene::Scene;
use crate::sphere::Sphere;

fn main() {
    let width = 960;
    let height = 540;

    let mut img = PngImage::new(width, height);

    let light = DirectionalLight::new(Vector3::new(1., -1., 1.), 1.0);

    let mut scene = Scene::new(light.clone());

    let diffuse_white = Diffuse(Color3::new(1., 1., 1.));
    let diffuse_blue = Diffuse(Color3::new(0.05, 0.5, 0.8));
    let diffuse_yellow = Diffuse(Color3::new(0.7, 0.8, 0.2));
    let specular_material = Specular;

    let skybox_color = Color3::new(0.3, 0.6, 0.8);
    let skybox = Rgb([(skybox_color.x * 256.) as u8, (skybox_color.y * 256.) as u8, (skybox_color.z * 256.) as u8]);

    scene.spheres.push(Sphere::new(Vector3::new(0.7, 1.0, 2.), 1.0, diffuse_white.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(-1.7, 0.2, 2.), 0.2, diffuse_blue.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(-0.4, 0.5, 1.6), 0.5, diffuse_yellow.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(1.0, 0.3, 1.), 0.3, diffuse_blue.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(0.4, 0.25, 1.), 0.25, specular_material.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(0., 0.1, 1.2), 0.1, specular_material.clone()));
    scene.spheres.push(Sphere::new(Vector3::new(0., -100000.0, 0.), 100000.0, diffuse_white.clone()));

    let camera = Camera::new(
        Vector3::new(0., 0.5, 0.),
        width as f64 / height as f64,
        60.0,
    );

    let brdf = 0.8;

    for y in 0..img.height {
        for x in 0..img.width {
            let half_width = img.width as f64 / 2.0;
            let half_height = img.height as f64 / 2.0;

            let u = (x as f64 - half_width) / half_width;
            let v = -(y as f64 - half_height) / half_height;

            let ray = camera.camera_ray(u, v);

            let color = trace(&scene, ray, 5);

            let rgb = Rgb([color.x as u8, color.y as u8, color.z as u8]);

            img.set_pixel(x, y, rgb);
        }
    }

    match std::fs::metadata("./images") {
        Err(_) => {
            std::fs::create_dir("./images");
        }
        Ok(_) => {}
    }

    match img.save("./images/image.png") {
        Err(E) => { println!("{:?}", E) }
        Ok(_) => { println!("done.") }
    };
}

fn trace(scene: &Scene, ray: Ray, reflection_time: i32) -> Color3 {
    let skybox_color = &Color3::new(0.3, 0.6, 0.8) * 256.;

    if reflection_time <= 0 { return Color3::zero(); }

    return match scene.intersect(ray.clone()) {
        None => skybox_color,
        Some(hit) => {
            let position = ray.clone().t(hit.t);
            let shadow_ray = Ray::new(&position + &((&hit.normal) * 0.01), &scene.light.clone().direction * -1.0);

            let brdf = 0.8;

            match scene.intersect(shadow_ray.clone()) {
                Some(_) => Color3::zero(),
                None => {
                    let cos_value = Vector3::dot(&shadow_ray.d, &hit.normal);
                    let radiance = cos_value * brdf * 256.0;
                    match hit.material {
                        Diffuse(col) => {
                            &col * radiance
                        }
                        Specular => {
                            let reflect_direction = &ray.d + &(&hit.normal * (Vector3::dot(&(&ray.d * -1.), &hit.normal) * 2.));
                            let hit_pos = &ray.t(hit.t)+ &(&hit.normal * 0.01);
                            let reflection_ray = Ray::new(hit_pos, reflect_direction);

                            return trace(scene, reflection_ray , reflection_time - 1);
                        }
                    }
                }
            }
        }
    };
}

