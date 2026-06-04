use super::Vector3;

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
        self.x.powi(2) + self.y.powi(2)
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
