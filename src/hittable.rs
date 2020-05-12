use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use crate::aabb::AABB;

use std::sync::Arc;
use crate::texture::SolidTexture;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            mat: Arc::new(Lambertian::new(
                 Arc::new(SolidTexture::new(0.0, 0.0, 0.0)))),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}
