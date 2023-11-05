use std::rc::Rc;

use three_d::{vec2, vec3, Camera, Vec2, Vec3};

use crate::embed::{CylindricalEmbedding, Embedding, OkhsvEmbedding};

pub struct InputState {
    pub mouse_pos: Vec2,
    pub color: Vec3,
    pub pos: Vec3,
    pub saved_pos: Vec3,
    pub chunk: Vec3,
    pub camera: Camera,
    pub press: bool,
    pub input: bool,
    pub color_embedding: Rc<dyn Embedding<Vec3>>,
    pub space_embedding: Rc<dyn Embedding<Vec3>>,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState {
            mouse_pos: vec2(0., 0.),
            color: vec3(0., 0., 0.),
            pos,
            saved_pos: pos,
            chunk: vec3(1., 1., 1.),
            camera,
            press: false,
            input: false,
            color_embedding: Rc::new(OkhsvEmbedding {}),
            space_embedding: Rc::new(CylindricalEmbedding {}),
        }
    }
}
