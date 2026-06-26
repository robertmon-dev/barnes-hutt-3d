use crate::vector::Vector3;

pub struct Correction {
    t: f32,
    v0: Vector3,
    v0_new: Vector3,
    position: Option<Vector3>,
    last_position: Option<Vector3>,
}

impl Correction {
    pub fn new(
        t: f32,
        v0: Vector3,
        v0_new: Vector3,
        position: Option<Vector3>,
        last_position: Option<Vector3>,
    ) -> Self {
        Self {
            t,
            v0,
            v0_new,
            position,
            last_position,
        }
    }
}
