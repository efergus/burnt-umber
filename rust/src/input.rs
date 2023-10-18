use three_d::{vec3, Camera, Vec3};

pub struct InputState {
    pub pos: Vec3,
    pub saved_pos: Vec3,
    pub chunk: Vec3,
    pub camera: Camera,
    pub input: bool,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState {
            pos,
            saved_pos: pos,
            chunk: vec3(1., 1., 1.),
            camera,
            input: false,
        }
    }
}
