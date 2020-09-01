use crate::hittable_list::HittableList;
use crate::sphere::{Sphere, MovingSphere};
use crate::material::{Lambertian, Metal, Dielectric, DiffuseLight};
use crate::texture::{SolidTexture, CheckerTexture, NoiseTexture, ImageTexture};
use crate::util;
use crate::vec3::Vec3;
use crate::aarect::{XYRect, XZRect, YZRect};
use crate::hittable::{FlipFace, Translate, RotateY, Hittable};
use crate::cube::Cube;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::bvh::BVHNode;

use std::sync::Arc;
use std::collections::HashMap;

lazy_static! {
    pub static ref SCENE_MAP: HashMap<String, fn(usize, usize) -> Scene> = {
        let mut map = HashMap::new();
        // map.insert("Book 1".to_string(), book1_scene as fn(usize, usize) -> Scene);
        // map.insert("Book 2".to_string(), book2_scene as fn(usize, usize) -> Scene);
        map.insert("Cornell Box".to_string(), cornell_scene as fn(usize, usize) -> Scene);
        map.insert("Cornell Box with Cubes".to_string(), cornell_cubes_scene as fn(usize, usize) -> Scene);
        map.insert("Cornell Box with Metal Cube".to_string(), cornell_metal_cube_scene as fn(usize, usize) -> Scene);
        map.insert("Cornell Box with Glass Sphere (SLOW)".to_string(), cornell_metal_cube_scene as fn(usize, usize) -> Scene);
        map
    };
}

pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera,
    pub background_color: Vec3,
    pub lights: Arc<dyn Hittable>,
}

pub fn book1_objects() -> HittableList {
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(SolidTexture::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * util::random_double(),
                0.2,
                b as f64 + 0.9 * util::random_double(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = SolidTexture::from(Vec3::random() * Vec3::random());
                    let center1 = &(center + Vec3::new(0.0, util::random_double_range(0.0, 0.5), 0.0));
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
                    let fuzz = util::random_double_range(0.0, 0.5);
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

pub fn book1_camera(width: usize, height: usize) -> Camera {
    let aspect_ratio = width as f64 / height as f64;
    let lookfrom = Vec3::new(12.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    Camera::new_timed(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}

pub fn book1_scene(width: usize, height: usize) -> Scene {
    Scene {
        objects: book1_objects(),
        camera: book1_camera(width, height),
        background_color: Vec3::new(0xdd as f64 / 255.0, 0xec as f64 / 255.0, 0xff as f64 / 255.0),
        lights: Arc::new(HittableList::new()),
    }
}

pub fn book2_objects() -> HittableList {
    let mut cubes1 = HittableList::new();

    let ground = Arc::new(Lambertian::new(SolidTexture::new(0.48, 0.83, 0.53)));
    let cubes_per_side = 20;
    for i in 0..cubes_per_side {
        for j in 0..cubes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = util::random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            cubes1.add(Arc::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone())))
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BVHNode::from_list(&mut cubes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(SolidTexture::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(light, 123.0, 423.0, 147.0, 412.0, 554.0)));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material =
        Arc::new(Lambertian::new(SolidTexture::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(&center1, &center2, 0.0, 1.0, 50.0, moving_sphere_material)));

    objects.add(Arc::new(Sphere::new(&Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, 150.0, 145.0), 50.0,
                                     Arc::new(Metal::new(&Vec3::new(0.8, 0.8, 0.9), 10.0)))));

    let boundry = Arc::new(Sphere::new(&Vec3::new(260.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    objects.add(boundry.clone());
    objects.add(Arc::new(ConstantMedium::new(boundry, SolidTexture::new(0.2, 0.4, 0.9), 0.2)));

    let boundry = Arc::new(Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    objects.add(Arc::new(ConstantMedium::new(boundry, SolidTexture::new(1.0, 1.0, 1.0), 0.0001)));

    let emat = Arc::new(Lambertian::new(ImageTexture::new("img_files/earthmap.jpg")));
    objects.add(Arc::new(Sphere::new(&Vec3::new(400.0, 200.0, 400.0), 100.0, emat)));
    let pertex = NoiseTexture::new(1.5);
    objects.add(Arc::new(Sphere::new(&Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new(pertex)))));

    let mut cubes2 = HittableList::new();

    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        cubes2.add(Arc::new(Sphere::new(&Vec3::random_range(0.0, 165.0), 10.0, white.clone())))
    }

    objects.add(Arc::new(
        Translate::new(
            RotateY::new(
                BVHNode::from_list(&mut cubes2, 0.0, 1.0),
                15.0),
            Vec3::new(-100.0, 270.0, 395.0))));

    objects
}

fn book2_scene(width: usize, height: usize) -> Scene {
    Scene {
        objects: book2_objects(),
        camera: book1_camera(width, height),
        background_color: Vec3::new(0.0, 0.0, 0.0),
        lights: Arc::new(HittableList::new()),
    }
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

pub fn earth(radius: f64) -> Sphere {
    let earth_texture = ImageTexture::new("img_files/earthmap.jpg");
    let earth_surface = Arc::new(DiffuseLight::new(earth_texture));
    Sphere::new(&Vec3::zero(), radius, earth_surface)
}

fn simple_light() -> HittableList {
    let mut objects = two_perlin_spheres();

    let difflight = Arc::new(DiffuseLight::new(SolidTexture::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(Sphere::new(&Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    objects.add(Arc::new(XYRect::new(difflight, 3.0, 5.0, 1.0, 3.0, -2.0)));

    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let light = Arc::new(DiffuseLight::new(SolidTexture::new(15.0, 15.0, 15.0)));
    let red = Arc::new(Lambertian::new(SolidTexture::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidTexture::new(0.12, 0.45, 0.15)));

    objects.add(Arc::new(FlipFace::new(Arc::new(YZRect::new(green, 0.0, 555.0, 0.0, 555.0, 555.0)))));
    objects.add(Arc::new(YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)))));
    objects.add(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(white, 0.0, 555.0, 0.0, 555.0, 555.0)))));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(light, 213.0, 343.0, 227.0, 332.0, 554.0)))));


    objects
}

pub fn cornell_with_cubes() -> HittableList {
    let mut objects = cornell_box();

    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));

    let cube1 = Cube::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let cube1 = RotateY::new(cube1, 15.0);
    let cube1 = Arc::new(Translate::new(cube1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(cube1);

    let cube2 = Cube::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white);
    let cube2 = RotateY::new(cube2, -18.0);
    let cube2 = Arc::new(Translate::new(cube2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(cube2);

    objects
}

pub fn cornell_with_metal_cube() -> HittableList {
    let mut objects = cornell_box();

    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));
    let aluminium = Arc::new(Metal::new(&Vec3::new(0.8, 0.85, 0.88), 0.0));

    let cube1 = Cube::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), aluminium);
    let cube1 = RotateY::new(cube1, 15.0);
    let cube1 = Arc::new(Translate::new(cube1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(cube1);

    let cube2 = Cube::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white);
    let cube2 = RotateY::new(cube2, -18.0);
    let cube2 = Arc::new(Translate::new(cube2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(cube2);

    objects
}

pub fn cornell_with_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(SolidTexture::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidTexture::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidTexture::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(SolidTexture::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(FlipFace::new(Arc::new(YZRect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)))));
    objects.add(Arc::new(YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(XZRect::new(light, 113.0, 443.0, 127.0, 432.0, 554.0)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)))));
    objects.add(Arc::new(XZRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)))));

    let cube1 = Cube::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let cube1 = RotateY::new(cube1, 15.0);
    let cube1 = Arc::new(Translate::new(cube1, Vec3::new(265.0, 0.0, 295.0)));

    let cube2 = Cube::new(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let cube2 = RotateY::new(cube2, -18.0);
    let cube2 = Arc::new(Translate::new(cube2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new(cube1, SolidTexture::new(0.0, 0.0, 0.0), 0.01)));
    objects.add(Arc::new(ConstantMedium::new(cube2, SolidTexture::new(1.0, 1.0, 1.0), 0.01)));

    objects
}

pub fn cornell_camera(width: usize, height: usize) -> Camera {
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
        1.0,
    )
}

pub fn cornell_scene(width: usize, height: usize) -> Scene {
    let mut lights = HittableList::new();
    let mat = Arc::new(Lambertian::new(SolidTexture::from(Vec3::zero())));
    lights.add(Arc::new(XZRect::new(mat, 213.0, 343.0, 227.0, 332.0, 554.0)));

    let world = cornell_box();

    Scene {
        objects: world,
        camera: cornell_camera(width, height),
        background_color: Vec3::zero(),
        lights: Arc::new(lights),
    }
}

pub fn cornell_cubes_scene(width: usize, height: usize) -> Scene {
    let mut lights = HittableList::new();
    let mat = Arc::new(Lambertian::new(SolidTexture::from(Vec3::zero())));
    lights.add(Arc::new(XZRect::new(mat, 213.0, 343.0, 227.0, 332.0, 554.0)));

    let world = cornell_with_cubes();

    Scene {
        objects: world,
        camera: cornell_camera(width, height),
        background_color: Vec3::zero(),
        lights: Arc::new(lights),
    }
}

pub fn cornell_metal_cube_scene(width: usize, height: usize) -> Scene {
    let mut lights = HittableList::new();
    let mat = Arc::new(Lambertian::new(SolidTexture::from(Vec3::zero())));
    lights.add(Arc::new(XZRect::new(mat, 213.0, 343.0, 227.0, 332.0, 554.0)));

    let world = cornell_with_metal_cube();

    Scene {
        objects: world,
        camera: cornell_camera(width, height),
        background_color: Vec3::zero(),
        lights: Arc::new(lights),
    }
}