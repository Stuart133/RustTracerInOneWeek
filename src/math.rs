use std::ops::Range;

use nalgebra::{Point3, Rotation3, Unit, Vector3};
use rand::{thread_rng, Rng};

pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Point = Point3<f64>;
pub type Rotation = Rotation3<f64>;

#[inline]
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

#[inline]
pub fn random_point(range: Range<f64>) -> Point {
    Point::new(
        thread_rng().gen_range(range.clone()),
        thread_rng().gen_range(range.clone()),
        thread_rng().gen_range(range),
    )
}

#[inline]
pub fn random_color(min: f64, max: f64) -> Color {
    Color::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

#[inline]
pub fn random_vector_range(min: f64, max: f64) -> Vector {
    Vector::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_in_unit_disk() -> Vector {
    loop {
        let p = Vector::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vector {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vector {
    *Unit::new_normalize(random_in_unit_sphere())
}

#[inline]
pub fn random_in_hemisphere(normal: &Vector) -> Vector {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[inline]
pub fn near_zero(vector: &Vector) -> bool {
    let s = 1e-8;

    f64::abs(vector.x) < s && f64::abs(vector.y) < s && f64::abs(vector.z) < s
}
