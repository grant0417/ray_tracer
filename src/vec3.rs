use crate::util::{clamp, random_double, random_double_range};

use std::io::{Write};
use std::ops::{Neg, Index, Add, Sub, Mul, IndexMut};
use std::io;
use std::f64;
use nalgebra::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub v: Vector3<f64>
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { v: Vector3::new(x, y, z) }
    }

    pub fn zero() -> Self {
        Vec3 { v: Vector3::new(0.0, 0.0, 0.0) }
    }

    pub fn x(&self) -> f64 { self.v[0] }
    pub fn y(&self) -> f64 { self.v[1] }
    pub fn z(&self) -> f64 { self.v[2] }

    pub fn length(&self) -> f64 {
        self.v.norm()
    }

    pub fn length_squared(&self) -> f64 {
        self.v.norm_squared()
    }

    /// PPM color writer
    pub fn write_color<T: Write>(&self, writer: &mut T, samples_per_pixel: usize) -> io::Result<()> {
        let color = self.return_color(samples_per_pixel);
        writer.write_all(format!("{} {} {}\n",
                                 color.0, color.1, color.2
        ).as_bytes())?;
        io::Result::Ok(())
    }

    pub fn return_color(&self, samples_per_pixel: usize) -> (u8, u8, u8) {
        let scale = 1.0 / samples_per_pixel as f64;

        let v = self.scale(scale).sqrt();

        let mut r = v.x();
        let mut g = v.y();
        let mut b = v.z();

        if r.is_nan() {
            r = 0.0;
            eprintln!("NAN")
        }
        if g.is_nan() {
            g = 0.0;
            eprintln!("NAN")
        }
        if b.is_nan() {
            b = 0.0;
            eprintln!("NAN")
        }

        ((256.0 * clamp(r, 0.0, 0.999)) as u8,
         (256.0 * clamp(g, 0.0, 0.999)) as u8,
         (256.0 * clamp(b, 0.0, 0.999)) as u8, )
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Vec3 { v: self.v.scale(scalar) }
    }

    pub fn div(&self, scalar: f64) -> Self {
        Vec3 { v: self.v.scale(1.0 / scalar) }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.v.dot(&rhs.v)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 { v: self.v.cross(&rhs.v) }
    }

    pub fn sqrt(&self) -> Self {
        Vec3::new(self.x().sqrt(), self.y().sqrt(), self.z().sqrt())
    }

    pub fn unit_vector(&self) -> Self {
        Vec3 { v: self.v.normalize() }
    }

    pub fn max(&self, val: f64) -> Self {
        Vec3::new(self.x().max(val), self.y().max(val), self.z().max(val))
    }

    pub fn min(&self, val: f64) -> Self {
        Vec3::new(self.x().min(val), self.y().min(val), self.z().min(val))
    }

    pub fn random() -> Self {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        let a = random_double_range(0.0, 2.0 * f64::consts::PI);
        let z = random_double_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double(), random_double(), 0.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - n.scale(2.0 * v.dot(n))
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(n);
        let r_out_parallel = (*uv + n.scale(cos_theta)).scale(etai_over_etat);
        let r_out_perp = n.scale(-(1.0 - r_out_parallel.length_squared()).sqrt());
        r_out_parallel + r_out_perp
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { v: self.v.neg() }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { v: self.v + rhs.v }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { v: self.v - rhs.v }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
