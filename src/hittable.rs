use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use crate::aabb::AABB;
use crate::texture::SolidTexture;

use std::sync::Arc;
use std::f64;

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
            front_face: false,
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
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct Translate<T>
    where T: Hittable {
    object: T,
    offset: Vec3,
}

impl<T> Translate<T>
    where T: Hittable {
    pub fn new(object: T, offset: Vec3) -> Self {
        Translate { object, offset }
    }
}

impl<T> Hittable for Translate<T>
    where T: Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&moved_r, t_min, t_max, rec) {
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

pub struct RotateY<T>
    where T: Hittable {
    object: T,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: AABB,
}

impl<T> RotateY<T>
    where T: Hittable {
    pub fn new(object: T, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::new_max();
        let has_box = object.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
        let mut max = [f64::INFINITY, f64::INFINITY, f64::INFINITY];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.min().x() + (1.0 - i as f64) * bbox.min.x();
                    let y = j as f64 * bbox.min().y() + (1.0 - j as f64) * bbox.min.y();
                    let z = k as f64 * bbox.min().z() + (1.0 - k as f64) * bbox.min.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * cos_theta * z;

                    let tester = [newx, y, newz];

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = AABB::new(
            &Vec3::new(min[0], min[1], min[2]),
            &Vec3::new(max[0], max[1], max[2]));

        RotateY { object, sin_theta, cos_theta, has_box, bbox }
    }
}

impl<T> Hittable for RotateY<T>
    where T: Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
}

pub struct FlipFace {
    object: Arc<dyn Hittable>
}

impl FlipFace {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        FlipFace { object }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.object.hit(r, t_min, t_max, rec) {
            return false;
        }

        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.object.bounding_box(t0, t1, output_box)
    }
}

