use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::util::random_double;
use crate::texture::Texture;

use std::f64;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, albedo: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        false
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _u: f64, _v: f64, _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

/// https://en.wikipedia.org/wiki/Schlick%27s_approximation
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

/// A material that models a perfectly diffuse surface that
/// scatters equally in all directions
/// https://en.wikipedia.org/wiki/Lambertian_reflectance
#[derive(Clone)]
pub struct Lambertian<T>
    where T: Texture {
    albedo: T,
}

impl<T> Lambertian<T>
    where T: Texture {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T> Material for Lambertian<T>
    where T: Texture {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, albedo: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, (target - rec.p).unit_vector(), r_in.time());
        *albedo = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = 0.5 / f64::consts::PI;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(&scattered.direction().unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f64::consts::PI
        }
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Vec3, f: f64) -> Self {
        Metal {
            albedo: *a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, albedo: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere().scale(self.fuzz), r_in.time());
        *albedo = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}


/// A dielectric material that models materials that reflect and refract
/// light based on an index of reflection
/// https://en.wikipedia.org/wiki/Dielectric
/// https://en.wikipedia.org/wiki/Snell%27s_law
#[derive(Clone)]

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, albedo: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        *albedo = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = 1.0_f64.min((-unit_direction).dot(&rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected, r_in.time());
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected, r_in.time());
            return true;
        }
        let refracted = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted, r_in.time());
        true
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T>
    where T: Texture {
    emit: T
}

impl<T> DiffuseLight<T>
    where T: Texture {
    pub fn new(emmisive_texture: T) -> Self {
        DiffuseLight { emit: emmisive_texture }
    }
}

impl<T> Material for DiffuseLight<T>
    where T: Texture{
    fn scatter(&self, _r_in: &Ray, _rec: &mut HitRecord, _albedo: &mut Vec3, _scattered: &mut Ray, pdf: &mut f64) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}

#[derive(Clone)]
pub struct Isotropic<T>
    where T: Texture {
    albedo: T
}

impl<T> Isotropic<T>
    where T: Texture {
    pub fn new(albedo: T) -> Self {
        Isotropic { albedo }
    }

    pub fn albedo(&self) -> T {
        self.albedo.clone()
    }
}

impl<T> Material for Isotropic<T>
    where T: Texture {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, albedo: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *albedo = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}