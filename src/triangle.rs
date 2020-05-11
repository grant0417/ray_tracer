use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;
use crate::aabb::AABB;

#[derive(Clone)]
pub enum Norm {
    Smooth(Vec3, Vec3, Vec3),
    Flat(Vec3)
}

#[derive(Clone)]
pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    e1: Vec3,
    e2: Vec3,
    norm: Norm,
    material: Arc<dyn Material>
}

impl Triangle {
    pub fn new_flat(p1: Vec3, p2: Vec3, p3: Vec3, material: Arc<dyn Material>) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let norm = e1.cross(&e2).unit_vector();
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            norm: Norm::Flat(norm),
            material
        }
    }

    pub fn new_smooth(p1: Vec3, p2: Vec3, p3: Vec3, n1: Vec3, n2: Vec3, n3: Vec3, material: Arc<dyn Material>) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            norm: Norm::Smooth(n1, n2, n3),
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
        let pos = r.at(t);

        if t > t_min && t < t_max {
            match self.norm {
                Norm::Smooth(n1, n2, n3) => {
                    rec.normal = n2.scale(u) + n3.scale(v) + n1.scale(1.0 - u - v);
                }
                Norm::Flat(norm) => {
                    rec.normal = norm;
                }
            }
            rec.t = t;
            rec.p = pos;
            rec.mat = self.material.clone();
            true
        } else {
            false
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let minx = self.p1.x().min(self.p2.x().min(self.p3.x()));
        let miny = self.p1.y().min(self.p2.y().min(self.p3.y()));
        let minz = self.p1.z().min(self.p2.z().min(self.p3.z()));

        let maxx= self.p1.x().max(self.p2.x().max(self.p3.x()));
        let maxy= self.p1.y().max(self.p2.y().max(self.p3.y()));
        let maxz= self.p1.z().max(self.p2.z().max(self.p3.z()));

        output_box.min = Vec3::new(minx, miny, minz);
        output_box.max = Vec3::new(maxx, maxy, maxz);

        true
    }
}