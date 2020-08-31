use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::vec3::Vec3;

use std::sync::Arc;
use crate::util::random_int_range;
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        HittableList { objects: Vec::with_capacity(capacity) }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() { return false; }

        let mut temp_box = AABB::new(&Vec3::zero(), &Vec3::zero());
        let mut first_box = true;

        for object in &self.objects {
            if object.bounding_box(t0, t1, &mut temp_box) { return false; }
            *output_box = if first_box { temp_box.clone() } else { output_box.surrounding_box(&temp_box) };
            first_box = false;
        }

        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;

        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        match self.objects.choose(&mut rng) {
            None => { Vec3::random() }
            Some(s) => { s.random(o) }
        }
    }
}
