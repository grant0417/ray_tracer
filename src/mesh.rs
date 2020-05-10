use crate::hittable_list::HittableList;
use std::path;
use crate::triangle::Triangle;
use crate::material::{Metal, Lambertian};
use crate::vec3::Vec3;
use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Mesh {
    list: HittableList,
}

impl Mesh {
    pub fn new_from_obj(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, materials) = tobj::load_obj(path)?;
        let mut list = HittableList::new();

        let scale = 0.3;

        let mat = Arc::new(Metal::new(&Vec3::new(0.0, 0.66, 0.42), 0.5));

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            for f in 0..mesh.indices.len() / 3 {
                let p1i = mesh.indices[3 * f] as usize;
                let p2i = mesh.indices[3 * f + 1] as usize;
                let p3i = mesh.indices[3 * f + 2] as usize;
                let p1 = Vec3::new(
                    mesh.positions[3 * p1i] as f64,
                    mesh.positions[3 * p1i + 1] as f64,
                    mesh.positions[3 * p1i + 2] as f64);
                let p2 = Vec3::new(
                    mesh.positions[3 * p2i] as f64,
                    mesh.positions[3 * p2i + 1] as f64,
                    mesh.positions[3 * p2i + 2] as f64);
                let p3 = Vec3::new(
                    mesh.positions[3 * p3i] as f64,
                    mesh.positions[3 * p3i + 1] as f64,
                    mesh.positions[3 * p3i + 2] as f64);

                list.add(Arc::new(Triangle::new(
                    p1.scale(scale),
                    p2.scale(scale),
                    p3.scale(scale),
                    mat.clone()
                )))
            }


        }

        Ok(Mesh{list})
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.list.hit(r, t_min, t_max, rec)
    }
}



