use crate::vec3::Vec3;
use crate::ray::Ray;

use std::f64;
use core::mem;

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: &Vec3, max: &Vec3) -> Self {
        AABB { min: *min, max: *max }
    }

    pub fn new_max() -> Self {
        AABB {
            min: Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn surrounding_box(&self, other: &AABB) -> Self {
        let small = Vec3::new(
            self.min().x().min(other.min().x()),
            self.min().y().min(other.min().y()),
            self.min().z().min(other.min().z()));
        let big = Vec3::new(
            self.max().x().max(other.max().x()),
            self.max().y().max(other.max().y()),
            self.max().z().max(other.max().z()));
        AABB { min: small, max: big }
    }

    pub fn add_point(&mut self, point: &Vec3) {
        let small = Vec3::new(
            self.min().x().min(point.x()),
            self.min().y().min(point.y()),
            self.min().z().min(point.z()));
        let big = Vec3::new(
            self.max().x().max(point.x()),
            self.max().y().max(point.y()),
            self.max().z().max(point.z()));
        self.min = small;
        self.max = big;
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, tmin: &mut f64, tmax: &mut f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            *tmin = if t0 > *tmin { t0 } else { *tmin };
            *tmax = if t1 < *tmax { t1 } else { *tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
