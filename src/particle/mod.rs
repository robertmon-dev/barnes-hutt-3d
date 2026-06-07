use crate::vector::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector3,
    pub last_position: Vector3,

    pub acceleration: Vector3,
    pub mass: f32,
    pub is_pinned: bool,
    pub radius: f32,
}

pub mod traits;

impl Particle {
    pub fn new(
        position: Vector3,
        last_position: Vector3,
        acceleration: Vector3,
        mass: f32,
        radius: f32,
    ) -> Self {
        Self {
            position,
            last_position,
            acceleration,
            mass,
            is_pinned: false,
            radius,
        }
    }
}

pub trait Moving {
    fn get_velocity(&self) -> Vector3;
    fn set_velocity(&mut self, vel: Vector3);
}

impl Moving for Particle {
    fn get_velocity(&self) -> Vector3 {
        self.position - self.last_position
    }

    fn set_velocity(&mut self, vel: Vector3) {
        self.last_position = self.position - vel;
    }
}
