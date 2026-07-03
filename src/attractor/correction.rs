use crate::vector::Vector3;

pub enum Correction {
    Collision {
        index: usize,
        t: f32,
        v0: Vector3,
        v0_new: Vector3,
        position: Option<Vector3>,
        last_position: Option<Vector3>,
    },
    Gravity {
        index: usize,
        acceleration: Vector3,
    },
    Kill {
        index: usize,
    },
}

impl Correction {
    pub fn from_acceleration(acceleration: Vector3, index: usize) -> Self {
        Correction::Gravity {
            index,
            acceleration,
        }
    }

    pub fn kill(index: usize) -> Self {
        Correction::Kill { index }
    }

    pub fn index(&self) -> usize {
        match self {
            Correction::Collision { index, .. } => *index,
            Correction::Gravity { index, .. } => *index,
            Correction::Kill { index, .. } => *index,
        }
    }
}
