use std::rc::Rc;

use cgmath::{vec3, SquareMatrix};
use three_d::{Context, Mat4, RenderStates, Vec3};

use crate::{
    embed::Embedding,
    input::InputState,
    mesh::{CpuMesh, Mesh},
};

use super::{ColorElement, ColorModel, ModelGraph};

pub struct ColorSpace {
    input: Mesh,
    space: Mesh,
    color: Mesh,
    color_embedding: Rc<dyn Embedding<Vec3>>,
    chunk: Vec3,
    view: Mat4,
}

impl ColorSpace {
    pub fn new(
        context: &Context,
        mesh: CpuMesh,
        space_embedding: Rc<dyn Embedding<Vec3>>,
        color_embedding: Rc<dyn Embedding<Vec3>>,
    ) -> Self {
        let split = mesh.split_triangles();
        let input = Mesh::new(context, split.clone());
        let positions = Mesh::from_mesh_embedded(context, &input, |pos| space_embedding.embed(pos));
        let embeded = Mesh::from_mesh_embedded(context, &input, |pos| color_embedding.embed(pos));

        ColorSpace {
            space: positions,
            input,
            color: embeded,
            color_embedding,
            chunk: vec3(1.0, 1.0, 1.0),
            view: Mat4::identity(),
        }
    }
}

impl ColorElement<InputState> for ColorSpace {
    fn update(&mut self, state: &InputState) {
        use cgmath::ElementWise;
        self.view = state.camera.projection() * state.camera.view();
        if state.chunk != self.chunk {
            self.color
                .embed_from_positions(self.input.positions(), |pos| {
                    self.color_embedding
                        .embed(pos.mul_element_wise(state.chunk))
                });
            self.chunk = state.chunk;
        }
    }

    fn update_state(&self, state: &mut InputState) {
        state.pos = self.invert_space(state.pos)
    }

    fn model(&self) -> ModelGraph {
        let model = Mat4::from_nonuniform_scale(self.chunk.z, self.chunk.y, self.chunk.z);
        ModelGraph::Vec(vec![
            ModelGraph::Color(ColorModel {
                positions: &self.space.vertex_buffer(),
                embed: &self.color.vertex_buffer(),
                indices: &self.space.element_buffer(),
                render_states: RenderStates::default(),
                view: self.view,
                model,
                meta: Mat4::identity(),
            }),
            ModelGraph::Space(ColorModel {
                positions: &self.space.vertex_buffer(),
                embed: &self.input.vertex_buffer(),
                indices: &self.space.element_buffer(),
                render_states: RenderStates::default(),
                view: self.view,
                model,
                meta: model,
            }),
        ])
    }
    fn invert_space(&self, pos: Vec3) -> Vec3 {
        pos
    }
}
