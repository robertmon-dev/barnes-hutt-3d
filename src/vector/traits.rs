use super::Vector3;

use rand::RngExt;
use std::f32::consts::PI;

pub trait VectorOps {
    type Scalar;

    fn expand_bits(v: u64) -> u64;
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn cross(&self, other: &Self) -> Self;
    fn magnitude(&self) -> Self::Scalar;
    fn normalize(&self) -> Self;
    fn angle_between<V: VectorOps<Scalar = f64>>(a: &V, b: &V) -> f64;
    fn distance_to(&self, other: &Self) -> f64;
    fn square(&self) -> f64;
    fn get_morton_code(&self, min_bound: Vector3, max_bound: Vector3) -> u64;
}

impl VectorOps for Vector3 {
    type Scalar = f64;

    #[inline(always)]
    fn expand_bits(mut v: u64) -> u64 {
        v &= 0x00000000001fffff;
        v = (v | v << 32) & 0x001f00000000ffff;
        v = (v | v << 16) & 0x001f0000ff0000ff;
        v = (v | v << 8) & 0x010f00f00f00f00f;
        v = (v | v << 4) & 0x10c30c30c30c30c3;
        v = (v | v << 2) & 0x1249249249249249;
        v
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn square(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Self::zero()
        }
    }

    fn angle_between<V: VectorOps<Scalar = f64>>(a: &V, b: &V) -> f64 {
        let dot = a.dot(b);
        let mag_product = a.magnitude() * b.magnitude();
        (dot / mag_product).acos()
    }

    fn distance_to(&self, other: &Self) -> f64 {
        let diff = *self - *other;
        diff.magnitude()
    }

    fn get_morton_code(&self, min_bounds: Vector3, max_bounds: Vector3) -> u64 {
        let max_val: f64 = ((1u64 << 21) - 1) as f64;

        let range_x = max_bounds.x - min_bounds.x;
        let range_y = max_bounds.y - min_bounds.y;
        let range_z = max_bounds.z - min_bounds.z;

        let nx = if range_x > 0.0 {
            ((self.x - min_bounds.x) / range_x).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let ny = if range_y > 0.0 {
            ((self.y - min_bounds.y) / range_y).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let nz = if range_z > 0.0 {
            ((self.z - min_bounds.z) / range_z).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let ix = (nx * max_val) as u64;
        let iy = (ny * max_val) as u64;
        let iz = (nz * max_val) as u64;

        let xx = Self::expand_bits(ix);
        let yy = Self::expand_bits(iy);
        let zz = Self::expand_bits(iz);

        (xx) | (yy << 1) | (zz << 2)
    }
}

pub trait Reflector {
    fn reflect(&self, normal: Vector3) -> Self;
}

impl Reflector for Vector3 {
    fn reflect(&self, norm: Vector3) -> Self {
        *self - 2.0 * self.dot(&norm) * norm
    }
}

pub trait Kinetic {
    fn kinetic_energy(&self, mass: f64) -> f64;
}

impl Kinetic for Vector3 {
    fn kinetic_energy(&self, mass: f64) -> f64 {
        self.magnitude().powi(2) * mass * 0.5
    }
}

pub trait Distributing {
    fn splash_coordinates_with_phi_epsilon(
        i: usize,
        n: usize,
        phi: f32,
        epsilon: f32,
        r: f32,
    ) -> Vector3;
    fn splash_coordinates(i: usize, n: usize, r: f32) -> Vector3;
    fn distribute_across_the_torus(n: usize, r_inn: f32, r_out: f32) -> Vec<Vector3>;
    fn distribute_across_the_sphere(n: usize, r: f32) -> Vec<Vector3>;
}

impl Distributing for Vector3 {
    fn splash_coordinates_with_phi_epsilon(
        i: usize,
        n: usize,
        phi: f32,
        epsilon: f32,
        r: f32,
    ) -> Vector3 {
        let i_f = i as f32;
        let n_f = n as f32;

        let x = i_f / phi;
        let y = (i_f + epsilon) / (n_f - 1.0 + 2.0 * epsilon);

        let x_i = 2.0 * PI * x;
        let y_i = (1.0 - 2.0 * y).acos();

        let x_c = (r * x_i.cos() * y_i.sin()) as f64;
        let y_c = (r * y_i.sin() * x_i.sin()) as f64;
        let z_c = (r * y_i.cos()) as f64;

        Vector3::new(x_c, y_c, z_c)
    }

    fn splash_coordinates(i: usize, n: usize, r: f32) -> Vector3 {
        let default_phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let default_epsilon = 0.0;
        Self::splash_coordinates_with_phi_epsilon(i, n, default_phi, default_epsilon, r)
    }

    fn distribute_across_the_sphere(n: usize, r: f32) -> Vec<Vector3> {
        let mut bodies = Vec::with_capacity(n);
        for i in 0..n {
            let position = Self::splash_coordinates(i, n, r);
            bodies.push(position);
        }

        bodies
    }

    fn distribute_across_the_torus(n: usize, r_inn: f32, r_out: f32) -> Vec<Vector3> {
        let mut bodies = Vec::with_capacity(n);
        let mut randomizer = rand::rng();

        for _ in 0..n {
            let theta = randomizer.random_range(0.0..=2.0 * PI);
            let phi = randomizer.random_range(0.0..=2.0 * PI);

            let x = ((r_out + r_inn * phi.cos()) * theta.cos()) as f64;
            let y = ((r_out + r_inn * phi.cos()) * theta.sin()) as f64;
            let z = (r_inn * phi.sin()) as f64;

            bodies.push(Vector3::new(x, y, z));
        }

        bodies
    }
}
