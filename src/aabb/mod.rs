use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub center: Vector3,
    pub half_dimension: f64,
}

impl Aabb {
    pub fn new(center: Vector3, half_dimension: f64) -> Self {
        Self {
            center,
            half_dimension,
        }
    }

    pub fn contains(&self, p: Vector3) -> bool {
        (p.x - self.center.x).abs() <= self.half_dimension
            && (p.y - self.center.y).abs() <= self.half_dimension
            && (p.z - self.center.z).abs() <= self.half_dimension
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        let dx = (self.center.x - other.center.x).abs();
        let dy = (self.center.y - other.center.y).abs();
        let dz = (self.center.z - other.center.z).abs();
        let limit = self.half_dimension + other.half_dimension;

        dx <= limit && dy <= limit && dz <= limit
    }

    pub fn get_min_point(&self) -> Vector3 {
        Vector3::new(
            self.center.x - self.half_dimension,
            self.center.y - self.half_dimension,
            self.center.z - self.half_dimension,
        )
    }

    pub fn get_max_point(&self) -> Vector3 {
        Vector3::new(
            self.center.x + self.half_dimension,
            self.center.y + self.half_dimension,
            self.center.z + self.half_dimension,
        )
    }

    pub fn get_bounds(&self) -> (Vector3, Vector3) {
        (self.get_min_point(), self.get_max_point())
    }
}
