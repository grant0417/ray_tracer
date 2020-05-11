use rand::distributions::{Distribution, Uniform};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min }
    if x > max { return max }
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