use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3,
               vfov: f64,
               aspect: f64, aperture: f64, focus_dist: f64) -> Self {

        let origin = lookfrom;
        let lens_radius = aperture / 2.0;

        let theta = vfov.to_radians();
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = origin
            - u.scale(half_width * focus_dist)
            - v.scale(half_height * focus_dist)
            - w.scale(focus_dist);
        let horizontal = u.scale(2.0*half_width * focus_dist);
        let vertical = v.scale(2.0*half_height * focus_dist);

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x()) + self.v.scale(rd.y());
        Ray::new(self.origin + offset,
                 self.lower_left_corner + self.horizontal.scale(s) + self.vertical.scale(t) - self.origin - offset)
    }
}