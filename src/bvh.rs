use crate::hittable::{Hittable, HitRecord};
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::util::random_int_range;
use crate::hittable_list::HittableList;

use rayon::slice::ParallelSliceMut;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb_box: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>,
           start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let axis = random_int_range(0,3) as usize;

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if box_compare(objects[start].clone(), objects[start+1].clone(), axis) {
                (objects[start].clone(), objects[start+1].clone())
            } else {
                (objects[start+1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].par_sort_by(|a, b| {
                if box_compare(a.clone(), b.clone(), axis) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mid = start + object_span/2;

            let left = Arc::new(BVHNode::new(objects, start, mid, time0, time1));
            let right = Arc::new(BVHNode::new(objects, mid, end, time0, time1));

            (left as Arc<dyn Hittable>, right as Arc<dyn Hittable>)
        };

        let mut box_left = AABB::new_max();
        let mut box_right = AABB::new_max();

        if !left.bounding_box(time0, time1, &mut box_left) ||
            !right.bounding_box(time0, time1, &mut box_right) {
            eprintln!("No bounding box in BVHNode constructor.")
        }

        let aabb_box = box_left.surrounding_box(&box_right);

        BVHNode { left, right, aabb_box }
    }

    pub fn from_list(objects: &mut HittableList, time0: f64, time1: f64) -> Self {
        let len = objects.objects.len();
        BVHNode::new(&mut objects.objects, 0, len, time0, time1)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        if !self.aabb_box.hit(r, &mut t_min, &mut t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.aabb_box.clone();
        true
    }
}

fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> bool {
    let mut box_a = AABB::new_max();
    let mut box_b = AABB::new_max();

    if !a.bounding_box(0.0, 0.0, &mut box_a) ||
        !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVHNode constructor.")
    }

    box_a.min()[axis] < box_b.min()[axis]
}
