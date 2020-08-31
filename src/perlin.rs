use crate::util::random_int_range;
use crate::vec3::Vec3;

const POINT_COUNT: usize = 256;

#[derive(Copy, Clone)]
pub struct Perlin {
    ran_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perlin = Perlin {
            ran_vec: [Vec3::zero(); POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };

        for i in perlin.ran_vec.iter_mut() {
            *i = Vec3::random_range(-1.0, 1.0)
        }

        perlin.perm_x = Self::perlin_generate_perm();
        perlin.perm_y = Self::perlin_generate_perm();
        perlin.perm_z = Self::perlin_generate_perm();

        perlin
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor();
        let j = point.y().floor();
        let k = point.z().floor();

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[(
                        self.perm_x[(i + di as f64) as usize & 255] ^
                            self.perm_y[(j + dj as f64) as usize & 255] ^
                            self.perm_z[(k + dk as f64) as usize & 255]) as usize];
                }
            }
        }

        Perlin::triliner_interp(c, u, v, w)
    }

    /// A composite noise based on the Perlin noise function
    /// Typical value for depth is 7
    pub fn turb(&self, point: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p.scale(2.0);
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];

        for (i, v) in p.iter_mut().enumerate() {
            *v = i as i32;
        }

        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
        for i in (0..n - 1).rev() {
            let target = random_int_range(0, (i + 1) as i64) as usize;
            p.swap(i, target);
        }
    }

    fn triliner_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                        (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                        (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) *
                        c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}