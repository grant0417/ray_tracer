use crate::vec3::Vec3;
use crate::onb::Onb;
use std::f64;
use crate::util;
use std::sync::Arc;
use crate::hittable::Hittable;

pub trait Pdf {
    fn value(&self, _direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        CosinePdf { uvw: Onb::build_from_w(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit_vector().dot(&self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / f64::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&util::random_cosine_direction())
    }
}

pub struct HittablePdf {
    o: Vec3,
    ptr: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new(o: Vec3, ptr: Arc<dyn Hittable>) -> Self {
        HittablePdf { o, ptr }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        (*self.ptr).pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        (*self.ptr).random(&self.o)
    }
}

pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2]
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        MixturePdf { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if util::random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}