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
                let outward_normal = (rec.p - self.center).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
        }
        false
    }
}


#[derive(Clone)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>
}

impl MovingSphere {
    pub fn new(center0: &Vec3, center1: &Vec3, time0: f64, time1: f64,
               radius: f64, mat: Arc<dyn Material>) -> Self {
        MovingSphere { center0: *center0, center1: *center1,
            time0, time1, radius, material: mat }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center0 + (self.center1 - self.center0)
            .scale((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
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
                let outward_normal = (rec.p - self.center(r.time())).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center(r.time())).div(self.radius);
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.material.clone();
                return true;
            }
        }
        false
    }
}