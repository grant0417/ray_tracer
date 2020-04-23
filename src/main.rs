mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod sphere;
use sphere::*;
mod hittable_list;
use hittable_list::*;
mod camera;
use camera::*;
mod util;
use util::*;
mod material;
use material::*;

use std::sync::{Arc, Mutex};
use std::{error::Error, io, thread};
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;
use std::time::Instant;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();
    if depth == 0 {
        return Vec3::zero();
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();

        if rec
            .mat
            .clone()
            .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3::zero();
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0).scale(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5))),
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
                    let albedo = Vec3::random() * Vec3::random();
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Lambertian::new(&albedo)),
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
        Arc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        &Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = 800;
    const SAMPLES_PER_PIXEL: usize = 1000;
    const MAX_DEPTH: usize = 50;

    let world = random_scene();
    let aspect_ratio = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let time = Instant::now();

    let mut positions = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            positions.push((i, j))
        }
    }

    let count = AtomicUsize::new(0);
    let total = IMAGE_HEIGHT * IMAGE_WIDTH;

    let colors: Vec<Vec3> = positions.par_iter().map(|(i,j)| {
        let mut color = Vec3::zero();
        for _ in 0..(SAMPLES_PER_PIXEL) {
            let u = (*i as f64 + random_double()) / IMAGE_WIDTH as f64;
            let v = (*j as f64 + random_double()) / IMAGE_HEIGHT as f64;
            let r = cam.get_ray(u, v);
            color += ray_color(&r, &world, MAX_DEPTH);
        }
        let count = count.fetch_add(1, Ordering::SeqCst);
        if count % 1000 == 0 {
            eprintln!("{:4.1}%", (count as f64 / total as f64) * 100.0);
        }
        color
    }).collect();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for color in colors {
        color.write_color(&mut io::stdout(), SAMPLES_PER_PIXEL)?;
    }

    eprintln!("\nDone in {} sec", time.elapsed().as_secs());

    Ok(())
}
