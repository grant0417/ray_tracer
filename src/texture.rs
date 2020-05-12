use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidTexture {
    color: Vec3,
}

impl SolidTexture {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        SolidTexture { color: Vec3::new(r, g, b) }
    }
}

impl From<Vec3> for SolidTexture {
    fn from(v: Vec3) -> Self {
        SolidTexture::new(v.x(), v.y(), v.z())
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>,
               odd: Arc<dyn Texture>) -> Self {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() 
                       * (10.0 * p.y()).sin() 
                       * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
