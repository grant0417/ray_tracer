pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod util;
pub mod material;
pub mod triangle;
pub mod mesh;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin;
pub mod aarect;
pub mod cube;
pub mod constant_medium;
pub mod scenes;
pub mod onb;
pub mod pdf;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::mesh::Mesh;
use crate::material::{Lambertian, ScatterRecord};
use std::sync::Arc;
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use crate::pdf::{HittablePdf, MixturePdf, Pdf};

const MAX_DEPTH: usize = 50;

pub fn ray_color<T: Hittable>(r: &Ray, background_color: &Vec3, world: &T, lights: Arc<dyn Hittable>, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();

    if depth == 0 {
        return Vec3::zero();
    }

    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background_color;
    }

    let mut srec = ScatterRecord::new();
    let emitted = rec.mat.emitted(r, &rec, rec.u, rec.v, &rec.p);

    if !rec.mat.clone().scatter(r, &mut rec, &mut srec) {
        return emitted;
    }

    if srec.is_specular {
        return srec.attenuation * ray_color(&srec.specular_ray, background_color, world, lights, depth - 1);
    }

    let light = Arc::new(HittablePdf::new(rec.p, lights.clone()));
    let p = Arc::new(MixturePdf::new(light, srec.pdf_ptr.unwrap()));

    let scattered = Ray::new(rec.p, p.generate(), r.time());
    let pdf_val = p.value(&scattered.direction());

    emitted + (srec.attenuation * ray_color(&scattered, background_color, world, lights, depth - 1))
        .scale(rec.mat.scattering_pdf(r, &rec, &scattered) / pdf_val)
}


pub fn render_scene(scene: &str, width: usize, height: usize, samples: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    //let dragon = Mesh::new_from_obj("obj_files/dragon_hq.obj", &Vec3::new(555.0/2.0, 0.0, 555.0/2.0), 250.0, false,
    //                                Arc::new(Lambertian::new(SolidTexture::new(0.5, 0.5, 0.5))))?;
    //world.add(Arc::new(dragon));

    let scene = scenes::SCENE_MAP.get(scene).unwrap()(width, height);

    eprintln!("Scene with {} objects.\n", scene.objects.objects.len());

    let mut positions = vec![0; 4 * width * height];

    if cfg!(target_os = "linux") {
        positions
            .par_chunks_mut(4)
            .enumerate()
            .for_each(|(i, chunk)| {
                let x = i % width;
                let y = height - i / width;

                let mut color = Vec3::zero();
                for _ in 0..(samples) {
                    let u = (x as f64 + util::random_double()) / width as f64;
                    let v = (y as f64 + util::random_double()) / height as f64;
                    let r = scene.camera.get_ray(u, v);
                    let ray_color = ray_color(&r, &scene.background_color, &scene.objects, scene.lights.clone(), MAX_DEPTH);

                    color = color + ray_color.min(1.0);
                }
                let rgb = color.return_color(samples);

                chunk[0] = rgb.0;
                chunk[1] = rgb.1;
                chunk[2] = rgb.2;
                chunk[3] = 255;
            });
    } else {
        positions
            .chunks_mut(4)
            .enumerate()
            .for_each(|(i, chunk)| {
                let x = i % width;
                let y = height - i / width;

                let mut color = Vec3::zero();
                for _ in 0..(samples) {
                    let u = (x as f64 + util::random_double()) / width as f64;
                    let v = (y as f64 + util::random_double()) / height as f64;
                    let r = scene.camera.get_ray(u, v);
                    let ray_color = ray_color(&r, &scene.background_color, &scene.objects, scene.lights.clone(), MAX_DEPTH);

                    color = color + ray_color.min(1.0);
                }
                let rgb = color.return_color(samples);

                chunk[0] = rgb.0;
                chunk[1] = rgb.1;
                chunk[2] = rgb.2;
                chunk[3] = 255;
            });
    }

    Ok(positions)
}

#[wasm_bindgen]
pub fn render_image_array(scene: &str, width: usize, height: usize, samples: usize) -> Box<[u8]> {
    let colors = render_scene(scene, width, height, samples).unwrap();

    colors.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_scenes() -> Box<[JsValue]> {
    let keys = scenes::SCENE_MAP.keys();
    let scenes: Vec<JsValue> = keys.map(|s| JsValue::from_str(s)).collect();
    scenes.into_boxed_slice()
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}