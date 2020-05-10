use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
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
}