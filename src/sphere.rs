use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Sphere { center: *center, radius, material: mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center).div(self.radius);
                let outward_normal = (rec.p - self.center).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center).div(self.radius);
                let outward_normal = (rec.p - self.center).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
        }
        false
    }
}