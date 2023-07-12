mod camera;
mod hittable;
mod material;
mod math;
mod objects;
mod ray;

use std::rc::Rc;

use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    math::Point,
    math::{Color, Vector},
    objects::Sphere,
};

// Quick hack to avoid floating point uncertainty causing self intersections
const MIN_INTERSECTION_DISTANCE: f64 = 0.0001;

const SAMPLES_PER_PIXEL: i64 = 100;
const MAX_DEPTH: i64 = 50;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: i64 = 400;
const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let lookfrom = Point::new(3.0, 3.0, 2.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).magnitude();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        2.0,
        dist_to_focus,
    );

    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}

fn write_color(pixel_color: Color, samples_per_pixel: i64) {
    let scale = 1.0 / samples_per_pixel as f64;

    // Average pixel samples and perform a quick gamma correction
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u64,
        (256.0 * g.clamp(0.0, 0.999)) as u64,
        (256.0 * b.clamp(0.0, 0.999)) as u64
    );
}
