use three_d::{vec2, vec3, Camera, Vec2, Vec3};

pub struct InputState {
    pub mouse_pos: Vec2,
    pub pos: Vec3,
    pub saved_pos: Vec3,
    pub chunk: Vec3,
    pub camera: Camera,
    pub input: bool,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState {
            mouse_pos: vec2(0., 0.),
            pos,
            saved_pos: pos,
            chunk: vec3(1., 1., 1.),
            camera,
            input: false,
        }
    }
}
