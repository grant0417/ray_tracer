use crate::hittable_list::HittableList;
use crate::triangle::Triangle;
use crate::material::{Material};
use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::bvh::BVHNode;

use std::sync::Arc;

pub struct Mesh {
    name: String,
    list: BVHNode,
}

impl Mesh {
    pub fn new_from_obj(path: &str, center: &Vec3, scale: f64, flat: bool, material: Arc<dyn Material>) -> Result<Self, tobj::LoadError> {
        let (models, _materials) = tobj::load_obj(path)?;
        let mut list = HittableList::new_with_capacity(models[0].mesh.indices.len()/3);

        let name = models[0].name.clone();

        for (_, m) in models.iter().enumerate() {
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

                if !mesh.normals.is_empty() && !flat {
                    let n1 = Vec3::new(
                        mesh.normals[3 * p1i] as f64,
                        mesh.normals[3 * p1i + 1] as f64,
                        mesh.normals[3 * p1i + 2] as f64);
                    let n2 = Vec3::new(
                        mesh.normals[3 * p2i] as f64,
                        mesh.normals[3 * p2i + 1] as f64,
                        mesh.normals[3 * p2i + 2] as f64);
                    let n3 = Vec3::new(
                        mesh.normals[3 * p3i] as f64,
                        mesh.normals[3 * p3i + 1] as f64,
                        mesh.normals[3 * p3i + 2] as f64);

                    list.add(Arc::new(Triangle::new_smooth(
                        p1.scale(scale) + *center,
                        p2.scale(scale) + *center,
                        p3.scale(scale) + *center,
                        n1,
                        n2,
                        n3,
                        material.clone()
                    )))
                } else {
                    list.add(Arc::new(Triangle::new_flat(
                        p1.scale(scale) + *center,
                        p2.scale(scale) + *center,
                        p3.scale(scale) + *center,
                        material.clone()
                    )))
                }
            }
        }

        eprintln!("Mesh '{}' imported with {} faces.", &name, list.objects.len());

        let mesh = Mesh{ name, list: BVHNode::from_list(&mut list, 0.0, 0.0) };

        eprintln!("Mesh '{}' bounded.", &mesh.name);

        Ok(mesh)
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.list.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.list.bounding_box(t0, t1, output_box)
    }
}
