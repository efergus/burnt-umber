use std::rc::Rc;

use cgmath::{vec3, SquareMatrix, Zero};
use three_d::{Context, Mat4, Vec3};

use crate::{embed::Embedding, geometry::quad_mesh, input::InputState, mesh::Mesh};

use super::{ColorElement, ColorModel, ModelGraph};

pub struct ColorChips {
    positions: Mesh,
    embed: Rc<dyn Embedding>,
    origin: Vec3,
    unit: Vec3,
    count: usize,
    size: f32,
}

impl ColorChips {
    pub fn new(context: &Context, count: usize, size: f32, embed: Rc<dyn Embedding>) -> Self {
        Self {
            positions: Mesh::from_positions(context, quad_mesh()),
            origin: Vec3::zero(),
            unit: vec3(0.5, -0.8, 0.0),
            embed,
            count,
            size,
        }
    }
}

impl ColorElement<InputState> for ColorChips {
    fn update(&mut self, state: &InputState) {
        self.origin = state.palette_pos;
    }

    fn model(&self) -> ModelGraph {
        let mut models = Vec::new();
        for i in 0..self.count {
            let pos = (self.origin + self.unit * (i as f32 / self.count as f32)).map(|x| {
                if x < 0.0 || x > 1.0 {
                    x.rem_euclid(1.0)
                } else {
                    x
                }
            });
            let view = Mat4::from_translation(vec3(
                1. - self.size,
                1. - (self.size * (i + 1) as f32),
                0.0,
            )) * Mat4::from_scale(self.size);
            models.push(ModelGraph::Color(ColorModel {
                positions: self.positions.vertex_buffer(),
                embed: self.positions.vertex_buffer(),
                indices: self.positions.element_buffer(),
                render_states: Default::default(),
                view,
                model: Mat4::identity(),
                meta: Mat4::from_translation(self.embed.embed(pos)) * Mat4::from_scale(0.0),
            }));
            models.push(ModelGraph::Space(ColorModel {
                positions: self.positions.vertex_buffer(),
                embed: self.positions.vertex_buffer(),
                indices: self.positions.element_buffer(),
                render_states: Default::default(),
                view,
                model: Mat4::identity(),
                meta: Mat4::from_translation(pos) * Mat4::from_scale(0.0),
            }));
        }
        ModelGraph::Vec(models)
    }

    fn invert_space(&self, pos: Vec3) -> Vec3 {
        pos
    }
}
