use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    e1: Vec3,
    e2: Vec3,
    norm: Vec3,
    material: Arc<dyn Material>
}

impl Triangle {
    pub(crate) fn new(p1: Vec3, p2: Vec3, p3: Vec3, material: Arc<dyn Material>) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let norm = e1.cross(&e2).unit_vector();
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            norm,
            material
        }
    }
}

impl Hittable for Triangle {
    // This implementation uses the Möller–Trumbore intersection algorithm
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        const EPSILON: f64 = 0.000_000_1;
        let h = r.direction().cross(&self.e2);
        let a = self.e1.dot(&h);
        if a > -EPSILON && a < EPSILON {
            return false;
        }
        let f = 1.0 / a;
        let s = r.origin() - self.p1;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return false;
        }
        let q = s.cross(&self.e1);
        let v = f * r.direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }
        let t = f * self.e2.dot(&q);

        if t > t_min && t < t_max {
            rec.normal = self.norm;
            rec.t = t;
            rec.p = r.at(t);
            rec.mat = self.material.clone();
            true
        } else {
            false
        }
    }
}