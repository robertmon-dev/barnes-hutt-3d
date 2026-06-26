use crate::vector::Vector3;

pub struct Correction {
    pub t: f32,
    pub v0: Vector3,
    pub v0_new: Vector3,
    pub position: Option<Vector3>,
    pub last_position: Option<Vector3>,
    pub index: usize,
}

impl Correction {
    pub fn new(
        t: f32,
        v0: Vector3,
        v0_new: Vector3,
        position: Option<Vector3>,
        last_position: Option<Vector3>,
        index: usize,
    ) -> Self {
        Self {
            t,
            v0,
            v0_new,
            position,
            last_position,
            index,
        }
    }
}
