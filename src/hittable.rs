use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use crate::aabb::AABB;
use crate::texture::SolidTexture;

use std::sync::Arc;

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
                 SolidTexture::new(0.0, 0.0, 0.0))),
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

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Translate { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new_timed(r.origin() - self.offset, r.direction(), r.time());

        if (!self.object.hit(&moved_r, t_min, t_max, rec)) {
            return false;
        }

        rec.p = rec.p + self.offset;
        let norm = rec.normal;
        rec.set_face_normal(&moved_r, &norm);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if !self.object.bounding_box(t0, t1, output_box) {
            return false;
        }

        *output_box = AABB::new(&(output_box.min + self.offset), &(output_box.max + self.offset));

        true
    }
}