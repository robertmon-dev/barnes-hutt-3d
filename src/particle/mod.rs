use crate::{attractor::correction::Correction, vector::Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector3,
    pub last_position: Vector3,

    pub acceleration: Vector3,
    pub mass: f32,
    pub is_pinned: bool,
    pub radius: f32,
    pub dead: bool,
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
            dead: false,
        }
    }

    pub fn extends(&self, min_bound: Vector3, max_bound: Vector3) -> bool {
        self.position.x >= min_bound.x + self.radius
            && self.position.x <= max_bound.x - self.radius
            && self.position.y >= min_bound.y + self.radius
            && self.position.y <= max_bound.y - self.radius
            && self.position.z >= min_bound.z + self.radius
            && self.position.z <= max_bound.z - self.radius
    }

    pub fn apply_correction(&mut self, correction: Correction) {
        if let Some(pos_offset) = correction.position {
            self.position += pos_offset;
        }

        if correction.v0_new != correction.v0 {
            self.last_position = self.position - correction.v0_new;
        } else if let Some(last_pos_offset) = correction.last_position {
            self.last_position += last_pos_offset;
        }
    }
}
