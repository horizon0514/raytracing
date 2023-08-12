use std::{fs, ops::AddAssign};
use std::io::Write;

mod vector3;
use vector3::{Color, Vector3, Point3};
mod color;
use color::write_color;

use crate::{ray3::{Ray3, ray_color}, util::random_double};
mod ray3;

mod sphere;
use sphere::Sphere;

mod hittable;
use hittable::Hittable;

mod hittable_list;
use hittable_list::HittableList;

mod camera;
use camera::Camera;

mod util;

mod material;
use material::{ Lambertian, Metal };

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    println!("image_width: {}, image_height: {}", image_width, image_height);

    // Camera, 位置在原点,朝向为负Z轴
    let camera = Camera::new(aspect_ratio);

    // Material
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    };

    // World
    let mut world = HittableList::new();
    let ground = Sphere::new(Vector3 { x: 0.0, y: -100.5, z: -1.0 }, 100.0, &material_ground);
    let sphere_center = Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, &material_center);
    let sphere_left = Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, 0.5, &material_left);
    let sphere_right = Sphere::new(Vector3 { x: 1.0, y: 0.0, z: -1.0 }, 0.5, &material_right);
    
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_right));
    world.add(Box::new(ground));
    // Render

    let file_name = "image.ppm";
    let mut file = fs::File::create(file_name).unwrap();
    // write ppm file header
    writeln!(&file, "P3").unwrap();
    writeln!(&file, "{} {}", image_width, image_height).unwrap();
    writeln!(&file, "255").unwrap();


    let sample_count = 100;
    let max_depth = 50;
    // write color 
    for  y in (0..image_height).rev() {
        print!("\rProgress: [{}%]", (image_height-y)/image_height*100); 

        for x in 0..image_width {
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                let u = ((x as f32) + util::random_double()) / (image_width-1) as f32;
                let v = (y as f32 + util::random_double()) / (image_height-1) as f32;
                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &world, max_depth);
            }
            
            write_color(&mut file, color, sample_count);
        }
    }
}
