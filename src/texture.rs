use crate::vec3::Vec3;

trait Texture {
    fn value(u: f64, v: f64, p: &Vec3) -> Vec3;
}
