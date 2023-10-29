use three_d::{vec2, vec3, Camera, Vec2, Vec3};

pub struct InputState {
    pub mouse_pos: Vec2,
    pub color: Vec3,
    pub pos: Vec3,
    pub palette_pos: Vec3,
    pub saved_pos: Vec3,
    pub chunk: Vec3,
    pub camera: Camera,
    pub input: bool,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState {
            mouse_pos: vec2(0., 0.),
            color: vec3(0., 0., 0.),
            pos,
            palette_pos: pos,
            saved_pos: pos,
            chunk: vec3(1., 1., 1.),
            camera,
            input: false,
        }
    }

    pub fn update_palette(&mut self) {
        self.palette_pos = self.pos;
    }
}
