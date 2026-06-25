use crate::vector::Vector3;

use super::Particle;

pub trait Moving {
    fn add_acceleration(&mut self, acc: Vector3);
    fn get_velocity(&self) -> Vector3;
    fn set_velocity(&mut self, vel: Vector3);
    fn set_acceleration(&mut self, acc: Vector3);
    fn update(&mut self, dt: f32);
    fn apply_force(&mut self, force: Vector3);
}

impl Moving for Particle {
    fn add_acceleration(&mut self, acc: Vector3) {
        self.acceleration += acc;
    }

    fn get_velocity(&self) -> Vector3 {
        self.position - self.last_position
    }

    fn set_velocity(&mut self, vel: Vector3) {
        self.last_position = self.position - vel;
    }

    fn set_acceleration(&mut self, acc: Vector3) {
        self.acceleration = acc;
    }

    fn update(&mut self, dt: f32) {
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
