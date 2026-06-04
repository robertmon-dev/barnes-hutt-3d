use crate::vector::Vector3;

use super::Particle;

pub trait Moving {
    fn update(&mut self, dt: f64);
    fn apply_force(&mut self, force: Vector3);
}

impl Moving for Particle {
    fn update(&mut self, dt: f64) {
        let velocity = self.position - self.last_position;
        let temp = self.position;

        self.position = self.position + velocity + self.acceleration * dt.powi(2);
        self.last_position = temp;

        self.acceleration = Vector3::zero();
    }

    fn apply_force(&mut self, force: Vector3) {
        self.acceleration += force / self.mass;
    }
}
