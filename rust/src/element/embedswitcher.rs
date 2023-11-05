use std::rc::Rc;

use cgmath::{vec3, SquareMatrix};
use three_d::{Context, Mat4, RenderStates, Vec3};

use crate::{
    element::coloraxis::Axis,
    embed::{
        AxisRepresentation, ChunkRepresentation, ComposedEmbedding, CylindricalEmbedding,
        Embedding, IdentityEmbedding, LinSrgbOklabEmbedding, OkhslEmbedding, OkhsvEmbedding,
        SwapAxesEmbedding,
    },
    input::InputState,
    mesh::Mesh,
    pre_embed::plane,
};

use super::{ColorElement, ColorModel, ModelGraph};

pub struct EmbedSwitcher {
    pub quad: Mesh,
    embeddings: Vec<Rc<dyn Embedding<Vec3>>>,
    index: usize,
    color: bool,
    pos: f32,
}

impl EmbedSwitcher {
    pub fn new(context: &Context, switch_color: bool, pos: f32) -> Self {
        let embeddings: Vec<Rc<dyn Embedding>> = if switch_color {
            vec![
                Rc::new(OkhsvEmbedding {}),
                Rc::new(OkhslEmbedding {}),
                Rc::new(LinSrgbOklabEmbedding {}),
                Rc::new(CylindricalEmbedding {}),
                Rc::new(IdentityEmbedding {}),
            ]
        } else {
            vec![
                Rc::new(CylindricalEmbedding {}),
                Rc::new(IdentityEmbedding {}),
                Rc::new(ComposedEmbedding::new(
                    Box::new(SwapAxesEmbedding::new(Axis::X, Axis::Y)),
                    Box::new(LinSrgbOklabEmbedding {}),
                    AxisRepresentation::Cylindrical,
                    ChunkRepresentation::Clamp,
                )),
            ]
        };
        let quad = Mesh::new(
            context,
            plane(1, 1, Vec3::unit_x(), Vec3::unit_y(), vec3(0.0, 0.0, 0.0)),
        );
        Self {
            quad,
            color: switch_color,
            embeddings,
            index: 0,
            pos,
        }
    }
}

impl ColorElement<InputState> for EmbedSwitcher {
    fn update(&mut self, _state: &InputState) {}

    fn clicked(&mut self) {
        self.index = (self.index + 1) % self.embeddings.len();
    }

    fn update_state(&self, state: &mut InputState) {
        if self.color {
            if !Rc::<dyn Embedding>::ptr_eq(&state.color_embedding, &self.embeddings[self.index]) {
                state.color_embedding = self.embeddings[self.index].clone();
            }
        } else {
            if !Rc::<dyn Embedding>::ptr_eq(&state.space_embedding, &self.embeddings[self.index]) {
                state.space_embedding = self.embeddings[self.index].clone();
            }
        }
    }

    fn model(&self) -> ModelGraph {
        let view = Mat4::from_translation(vec3(-1.0, 0.8 - self.pos, 0.0)) * Mat4::from_scale(0.2);
        ModelGraph::Vec(vec![
            ModelGraph::Color(ColorModel {
                positions: &self.quad.vertex_buffer(),
                embed: &self.quad.vertex_buffer(),
                indices: &self.quad.element_buffer(),
                render_states: RenderStates {
                    depth_test: three_d::DepthTest::Always,
                    ..Default::default()
                },
                view: view,
                model: Mat4::identity(),
                meta: Mat4::from_scale(0.0),
            }),
            ModelGraph::Space(ColorModel {
                positions: &self.quad.vertex_buffer(),
                embed: &self.quad.vertex_buffer(),
                indices: &self.quad.element_buffer(),
                render_states: RenderStates {
                    depth_test: three_d::DepthTest::Always,
                    ..Default::default()
                },
                view: view,
                model: Mat4::identity(),
                meta: Mat4::from_scale(0.0),
            }),
        ])
    }
}
