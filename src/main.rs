mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod util;
mod material;
mod triangle;
mod mesh;
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod aarect;
mod cube;
mod constant_medium;
mod scenes;

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::{error::Error};
use rayon::prelude::*;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{App, Arg};

fn ray_color<T: Hittable>(r: &Ray, background_color: &Vec3, world: &T, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();

    if depth == 0 {
        return Vec3::zero();
    }

    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background_color;
    }

    let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
    let emitted = rec.mat.emitted(rec.u, rec.v, &rec.p);
    let mut albedo = Vec3::zero();
    let mut pdf = 0.0;

    if !rec.mat.clone().scatter(r, &mut rec, &mut albedo, &mut scattered, &mut pdf) {
        return emitted;
    }

    emitted + albedo.scale(rec.mat.scattering_pdf(r, &rec, &scattered) / pdf)
        * ray_color(&scattered, background_color, world, depth - 1)

}

fn main() -> Result<(), Box<dyn Error>> {
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = 800;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 3;

    let width_default = IMAGE_WIDTH.to_string();
    let height_default = IMAGE_HEIGHT.to_string();
    let samples_default = SAMPLES_PER_PIXEL.to_string();

    let matches = App::new("Ray Tracer")
        .version("0.1")
        .author("Grant Gurvis")
        .about("A small ray tracer based on Ray Tracing in a weekend")
        .arg(Arg::with_name("OUTPUT")
            .help("The file to output to with a file extension")
            .required(true)
            .index(1))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Sets the render width")
            .default_value(&width_default)
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Sets the render height")
            .default_value(&height_default)
            .takes_value(true))
        .arg(Arg::with_name("samples")
            .short("s")
            .long("samples")
            .help("Sets the number of samples per pixel")
            .default_value(&samples_default)
            .takes_value(true))
        .get_matches();

    let file = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("width").unwrap().parse().unwrap_or(IMAGE_WIDTH);
    let height = matches.value_of("height").unwrap().parse().unwrap_or(IMAGE_HEIGHT);
    let samples = matches.value_of("samples").unwrap().parse().unwrap_or(SAMPLES_PER_PIXEL);

    eprintln!("Starting render.");
    eprintln!("Dimensions: {}x{}", width, height);
    eprintln!("Samples per Pixel: {}\n", samples);

    let _ext = image::ImageFormat::from_path(file)?;

    let time = Instant::now();

    //let dragon = Mesh::new_from_obj("dragon.obj", &Vec3::new(555.0/2.0, 0.0, 555.0/2.0), 250.0, false,
    //                                Arc::new(Metal::new(&Vec3::new(0.3125, 0.78125, 0.42), 0.46875)))?;

    let mut world = scenes::cornell_with_cubes();

    //world.add(Arc::new(Sphere::new(&Vec3::new(250.0, 250.0, 250.0), 50.0, Arc::new(Dielectric::new(1.5)))));

    //world.add(Arc::new(dragon));

    //eprintln!("Scene with {} objects.\n", &world.objects.len());

    let cam = scenes::cornell_camera(width, height);

    let mut positions = Vec::with_capacity(height * width);
        for j in (0..height).rev() {
            for i in 0..width {
            positions.push((i, j))
        }
    }

    let total = height * width;

    eprintln!("Rendering scene...");

    let render_bar = ProgressBar::new(total as u64);
    render_bar.set_style(ProgressStyle::default_bar()
        .template("{wide_bar} {percent}% Elapsed: {elapsed_precise} Remaining: {eta_precise}"));
    render_bar.set_draw_delta((total / 1000) as u64);

    let colors: Vec<Vec3> = positions.par_iter().map(|(i,j)| {
        let mut color = Vec3::zero();
        for _ in 0..(samples) {
            let u = (*i as f64 + util::random_double()) / width as f64;
            let v = (*j as f64 + util::random_double()) / height as f64;
            let r = cam.get_ray(u, v);
            color = color + ray_color(&r, &Vec3::new(0.0, 0.0, 0.0), &world, MAX_DEPTH);
        }
        render_bar.inc(1);
        color
    }).collect();

    render_bar.finish();

    eprintln!("\nOutputting to '{}'...", file);

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = colors[x as usize + y as usize * width].return_color(samples);
        *pixel = image::Rgb([color.0 as u8, color.1 as u8, color.2 as u8]);
    }

    imgbuf.save(file)?;

    eprintln!("\nDone in {}.{:03} sec", time.elapsed().as_secs(), time.elapsed().subsec_millis());

    Ok(())
}
