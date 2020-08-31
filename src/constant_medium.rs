use crate::hittable::{Hittable, HitRecord};
use crate::material::Isotropic;
use crate::ray::Ray;
use crate::aabb::AABB;

use std::sync::Arc;
use std::f64;
use crate::util::random_double;
use crate::vec3::Vec3;
use crate::texture::Texture;

pub struct ConstantMedium<T>
    where T: Texture {
    boundary: Arc<dyn Hittable>,
    phase_function: Isotropic<T>,
    neg_inv_density: f64,
}

impl<T> ConstantMedium<T>
    where T: Texture {
    pub fn new(boundrary: Arc<dyn Hittable>, albedo: T, density: f64) -> Self
        where T: Texture {
        let neg_inv_density = -1.0 / density;
        let phase_function = Isotropic::new(albedo);
        ConstantMedium { boundary: boundrary, phase_function, neg_inv_density }
    }
}

impl<T: 'static> Hittable for ConstantMedium<T>
    where T: Texture {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.001, f64::INFINITY, &mut rec2) {
            return false;
        }

        if rec1.t < t_min { rec1.t = t_min }
        if rec2.t > t_max { rec2.t = t_max }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundry = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundry {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Arc::new(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(t0, t1, output_box)
    }
}
