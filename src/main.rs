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

use crate::hittable::{Hittable, HitRecord, Translate};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::sphere::{Sphere, MovingSphere};
use crate::material::{Lambertian, Metal, Dielectric, DiffuseLight};
use crate::util::{random_double, random_double_range};
use crate::camera::Camera;

use std::sync::Arc;
use std::{error::Error};
use rayon::prelude::*;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{App, Arg};
use crate::texture::{SolidTexture, CheckerTexture, NoiseTexture, ImageTexture};
use crate::aarect::{XYRect, YZRect, XZRect};
use crate::cube::Cube;
use crate::mesh::Mesh;

fn ray_color<T: Hittable>(r: &Ray, background_color: &Vec3, world: &T, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();

    if depth == 0 {
        return Vec3::zero();
    }

    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background_color;
    }

    let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
    let mut attenuation = Vec3::zero();
    let emitted = rec.mat.emitted(rec.u, rec.v, &rec.p);

    if !rec.mat.clone().scatter(r, &mut rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, background_color, world, depth - 1)
}

fn book1_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(SolidTexture::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = SolidTexture::from(Vec3::random() * Vec3::random());
                    let center1 = &(center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0));
                    world.add(Arc::new(MovingSphere::new(
                        &center,
                        center1,
                        0.0, 1.0,
                        0.2,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat > 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Metal::new(&albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        &Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        &Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(SolidTexture::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        &Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new_with_capacity(2);

    let checker = Arc::new(Lambertian::new(
        CheckerTexture::new(
        SolidTexture::new(0.2, 0.3, 0.1),
        SolidTexture::new(0.9, 0.9, 0.9))));

    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, -10.0, 0.0), 10.0, checker.clone())));
    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, 10.0, 0.0), 10.0, checker)));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new_with_capacity(2);

    let checker = Arc::new(Lambertian::new(NoiseTexture::new(8.0)));

    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, -1000.0, 0.0), 1000.0, checker.clone())));
    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, 2.0, 0.0), 2.0, checker)));

    objects
}

fn earth() -> HittableList {
    let mut objects = HittableList::new_with_capacity(2);

    let earth_texture = ImageTexture::new("img_files/earthmap.jpg");
    let earth_surface = Arc::new(DiffuseLight::new(earth_texture));
    let globe = Arc::new(Sphere::new(&Vec3::zero(), 2.0, earth_surface));
    objects.add(globe);

    objects
}

fn simple_light() -> HittableList {
    let mut objects = two_perlin_spheres();

    let difflight = Arc::new(DiffuseLight::new(SolidTexture::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    objects.add(Arc::new(XYRect::new(difflight, 3.0, 5.0, 1.0, 3.0, -2.0, )));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(SolidTexture::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidTexture::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(SolidTexture::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(XZRect::new(light, 213.0, 343.0, 227.0, 332.0, 554.0)));
    objects.add(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(XYRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    // objects.add(Arc::new(Cube::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), white.clone())));
    // objects.add(Arc::new(Cube::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0), white.clone())));

    objects
}

fn cornell_camera(width: usize, height: usize) -> Camera {
    let aspect_ratio = width as f64 / height as f64;
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    Camera::new_timed(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0
    )
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

    let time = Instant::now();


    let dragon = Mesh::new_from_obj("obj_files/dragon_hack.obj", &Vec3::new(555.0/2.0, 0.0, 555.0/2.0), 200.0, false,
                                    Arc::new(Metal::new(&Vec3::new(0.3125, 0.78125, 0.42), 0.46875)))?;

    //let teapot = Mesh::new_from_obj("obj_files/teapot.obj",
    //                                &Vec3::new(278.0, 278.0, 0.0), 1.0, false,
    //                                Arc::new(Lambertian::new(SolidTexture::new(0.8, 0.8, 0.8))))?;

    let mut world = cornell_box();

    world.add(Arc::new(dragon));
    //world.add(Arc::new(teapot));

    eprintln!("Scene with {} objects.\n", &world.objects.len());

    let aspect_ratio = width as f64 / height as f64;
    let lookfrom = Vec3::new(-3.0, 1.5, 5.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).length();
    let aperture = 0.1;
    let cam = Camera::new_timed(
        lookfrom,
        lookat,
        vup,
        90.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0
    );

    let cam = cornell_camera(width, height);


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
            let u = (*i as f64 + random_double()) / width as f64;
            let v = (*j as f64 + random_double()) / height as f64;
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
