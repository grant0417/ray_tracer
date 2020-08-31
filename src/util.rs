use rand::distributions::{Distribution, Uniform};
use crate::vec3::Vec3;
use std::f64;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

pub fn random_double() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    if min.ne(&max) {
        let between = Uniform::from(min..max);
        let mut rng = rand::thread_rng();
        between.sample(&mut rng)
    } else {
        min
    }
}

pub fn random_int_range(min: i64, max: i64) -> i64 {
    let between = Uniform::from(min..max);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
