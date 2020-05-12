use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::material::Material;

use std::sync::Arc;
use crate::aarect::{XYRect, XZRect, YZRect};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;

pub struct Cube {
    cube_min: Vec3,
    cube_max: Vec3,
    sides: HittableList,
}

impl Cube {
    pub fn new(cube_min: Vec3, cube_max: Vec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            material.clone(), cube_min.x(), cube_max.x(), cube_min.y(), cube_max.y(), cube_max.z())));
        sides.add(Arc::new(XYRect::new(
            material.clone(), cube_min.x(), cube_max.x(), cube_min.y(), cube_max.y(), cube_min.z())));

        sides.add(Arc::new(XZRect::new(
            material.clone(), cube_min.x(), cube_max.x(), cube_min.z(), cube_max.z(), cube_max.y())));
        sides.add(Arc::new(XZRect::new(
            material.clone(), cube_min.x(), cube_max.x(), cube_min.z(), cube_max.z(), cube_min.y())));

        sides.add(Arc::new(YZRect::new(
            material.clone(), cube_min.y(), cube_max.y(), cube_min.z(), cube_max.z(), cube_max.x())));
        sides.add(Arc::new(YZRect::new(
            material.clone(), cube_min.y(), cube_max.y(), cube_min.z(), cube_max.z(), cube_min.x())));

        Cube { cube_min, cube_max, sides }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(&self.cube_min, &self.cube_max);
        true
    }
}