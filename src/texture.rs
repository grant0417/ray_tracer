use crate::vec3::Vec3;
use crate::perlin::Perlin;
use crate::util::clamp;

use image::GenericImageView;

pub trait Texture: Sync + Send + Clone {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct CheckerTexture<T, U>
    where T: Texture, U: Texture {
    even: T,
    odd: U,
}

impl<T, U> CheckerTexture<T, U>
    where T: Texture, U: Texture {
    pub fn new(even: T,
               odd: U) -> Self {
        CheckerTexture { even, odd }
    }
}

impl<T, U> Texture for CheckerTexture<T, U>
    where T: Texture, U: Texture {
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

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture { noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            .scale(0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&p, 7)).sin()))
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    width: u32,
    height: u32,
    image: image::DynamicImage,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let img = image::open(path).unwrap();
        ImageTexture {
            width: img.dimensions().0,
            height: img.dimensions().1,
            image: img,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        if i >= self.width { i = self.width - 1 }
        if j >= self.height { j = self.height - 1 }

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);

        Vec3::new(color_scale * pixel.0[0] as f64,
                  color_scale * pixel.0[1] as f64,
                  color_scale * pixel.0[2] as f64)
    }
}