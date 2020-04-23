use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::util::random_double;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: &Vec3) -> Self {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let scattered_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scattered_direction);
        *attenuation = self.albedo;
        true
    }
}

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
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere().scale(self.fuzz));
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = 1.0_f64.min(-unit_direction.dot(&rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }
        let refracted = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}