use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new() -> Onb {
        Onb { axis: [Vec3::zero(); 3] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u().scale(a) + self.v().scale(b) + self.w().scale(c)
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        self.u().scale(a.x()) + self.v().scale(a.y()) + self.w().scale(a.z())
    }

    pub fn build_from_w(n: &Vec3) -> Onb {
        let w = n.unit_vector();
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit_vector();
        let u = w.cross(&v);

        Onb { axis: [w, v, u] }
    }
}