use std::rc::Rc;

use cgmath::{SquareMatrix, Zero};
use three_d::{Context, Mat4, RenderStates, Vec3};

use crate::{
    embed::{ChunkRepresentation, Embedding, IdentityEmbedding},
    input::InputState,
    mesh::{CpuMesh, Mesh},
};

use super::{ColorElement, ColorModel, ModelGraph};

pub struct ColorSpace {
    input: Mesh,
    space: Mesh,
    color: Mesh,
    chunk: Vec3,
    view: Mat4,
    color_embedding: Rc<dyn Embedding<Vec3>>,
    space_embedding: Rc<dyn Embedding<Vec3>>,
    representation: ChunkRepresentation,
}

impl ColorSpace {
    pub fn new(context: &Context, mesh: CpuMesh) -> Self {
        let split = mesh.split_triangles();
        let input = Mesh::new(context, split.clone());
        let positions = Mesh::new(context, split.clone());
        let embeded = Mesh::new(context, split.clone());

        ColorSpace {
            space: positions,
            input,
            color: embeded,
            chunk: Vec3::zero(),
            view: Mat4::identity(),
            color_embedding: Rc::new(IdentityEmbedding {}),
            space_embedding: Rc::new(IdentityEmbedding {}),
            representation: ChunkRepresentation::Scale,
        }
    }
}

impl ColorElement<InputState> for ColorSpace {
    fn update(&mut self, state: &InputState) {
        use cgmath::ElementWise;
        self.view = state.camera.projection() * state.camera.view();
        let representation = state.space_embedding.chunk_representation();
        if state.chunk != self.chunk
            || !Rc::<dyn Embedding>::ptr_eq(&self.color_embedding, &state.color_embedding)
            || self.representation != representation
        {
            if representation == ChunkRepresentation::Clamp {
                self.color
                    .embed_from_positions(self.input.positions(), |pos| {
                        state
                            .color_embedding
                            .embed(pos)
                            .zip(self.chunk, |p, c| p.min(c))
                    });
                self.space
                    .embed_from_positions(self.input.positions(), |pos| {
                        state
                            .space_embedding
                            .embed(pos)
                            .zip(self.chunk, |p, c| p.min(c))
                    });
            } else {
                self.color
                    .embed_from_positions(self.input.positions(), |pos| {
                        state
                            .color_embedding
                            .embed(pos.mul_element_wise(state.chunk))
                    });
            }
            self.color_embedding = state.color_embedding.clone();
            self.chunk = state.chunk;
            self.representation = representation;
        }
        if !Rc::<dyn Embedding>::ptr_eq(&self.space_embedding, &state.space_embedding) {
            self.space
                .embed_from_positions(self.input.positions(), |pos| {
                    state.space_embedding.embed(pos)
                });
            self.space_embedding = state.space_embedding.clone();
        }
    }

    fn model(&self) -> ModelGraph {
        let model = match self.representation {
            ChunkRepresentation::Clamp => Mat4::identity(),
            ChunkRepresentation::Scale => {
                Mat4::from_nonuniform_scale(self.chunk.x, self.chunk.y, self.chunk.z)
            }
        };
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
    fn invert_space(&self, pos: Vec3) -> Option<Vec3> {
        Some(pos)
    }
}
