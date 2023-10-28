use std::rc::Rc;

use cgmath::{vec3, ElementWise, InnerSpace, SquareMatrix, Zero};
use three_d::{Context, Mat4, RenderStates, Vec3};

use crate::{embed::Embedding, input::InputState, mesh::Mesh, pre_embed::plane};

use super::{ColorElement, ColorModel, ModelGraph};

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
pub struct ColorAxis {
    pub positions: Mesh,
    // pub cursor_positions: Mesh,
    pub input: Mesh,
    pub embed: Mesh,
    pub axis: Axis,
    color_embedding: Rc<dyn Embedding<Vec3>>,
    // space_embedding: Rc<dyn Embedding<Vec3>>,
    pos: Vec3,
}

impl ColorAxis {
    pub fn new(
        context: &Context,
        axis: Axis,
        color_embedding: Rc<dyn Embedding<Vec3>>,
        // space_embedding: Rc<dyn Embedding<Vec3>>,
    ) -> ColorAxis {
        let unit_x = match axis {
            Axis::X | Axis::Y => vec3(1.0, 0.0, 0.0),
            Axis::Z => vec3(0.0, 0.0, 1.0),
        };
        let unit_y = vec3(0.0, 1.0, 0.0);
        let (horizontal_subdivisions, vertical_subdivisions) = match axis {
            Axis::X | Axis::Z => (16, 1),
            Axis::Y => (1, 16),
        };

        let mesh = Mesh::new(
            context,
            plane(
                horizontal_subdivisions,
                vertical_subdivisions,
                unit_x,
                unit_y,
                vec3(0.0, 0.0, 0.0),
            ),
        );
        let positions = Mesh::from_mesh_embedded(context, &mesh, |pos| {
            vec3(pos.dot(unit_x), pos.dot(unit_y), 0.0)
        });
        // let cursor_positions = Mesh::from_mesh_embedded(context, &mesh, |pos| {
        //     space_embedding.embed(pos)
        // });

        ColorAxis {
            positions,
            // cursor_positions,
            input: Mesh::from_mesh(context, &mesh),
            embed: Mesh::from_mesh(context, &mesh),
            axis,
            color_embedding,
            // space_embedding,
            pos: Vec3::zero(),
        }
    }

    fn view_matrix(&self) -> Mat4 {
        match self.axis {
            Axis::X => {
                Mat4::from_translation(vec3(-0.5, 0.8, 0.0))
                    * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0)
            }
            Axis::Y => {
                Mat4::from_translation(vec3(-1.0, -0.5, 0.0))
                    * Mat4::from_nonuniform_scale(0.2, 1.0, 1.0)
            }
            Axis::Z => {
                Mat4::from_translation(vec3(-0.5, -1.0, 0.0))
                    * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0)
            }
        }
    }
}

impl ColorElement<InputState> for ColorAxis {
    fn model(&self) -> ModelGraph {
        ModelGraph::Vec(vec![
            ModelGraph::Color(ColorModel {
                positions: self.positions.vertex_buffer(),
                embed: self.embed.vertex_buffer(),
                indices: self.positions.element_buffer(),
                render_states: RenderStates::default(),
                view: self.view_matrix(),
                model: Mat4::identity(),
                meta: Mat4::identity(),
            }),
            ModelGraph::Space(ColorModel {
                positions: self.positions.vertex_buffer(),
                embed: self.input.vertex_buffer(),
                indices: self.positions.element_buffer(),
                render_states: RenderStates::default(),
                view: self.view_matrix(),
                model: Mat4::identity(),
                meta: Mat4::identity(),
            }),
        ])
    }

    fn invert_space(&self, pos: Vec3) -> Vec3 {
        match self.axis {
            Axis::X => vec3(pos.x, self.pos.y, self.pos.z),
            Axis::Y => vec3(self.pos.x, pos.y, self.pos.z),
            Axis::Z => vec3(self.pos.x, self.pos.y, pos.z),
        }
    }

    fn update(&mut self, state: &InputState) {
        let pos = state.pos;
        self.pos = pos;

        let scale = match self.axis {
            Axis::X | Axis::Z => vec3(1.0, 0.0, 1.0),
            Axis::Y => vec3(0.0, 1.0, 0.0),
        };

        let origin = match self.axis {
            Axis::X => vec3(pos.x - 0.5, pos.y, pos.z),
            Axis::Y => vec3(pos.x, 0.0, pos.z),
            Axis::Z => vec3(pos.x, pos.y, 0.0),
        };

        self.embed
            .embed_from_positions(self.input.positions(), |pos| {
                self.color_embedding
                    .embed(pos.mul_element_wise(scale) + origin)
            });
    }

    fn update_state(&self, state: &mut InputState) {
        let chunk = state.chunk;
        state.chunk = match self.axis {
            Axis::X | Axis::Z => chunk,
            Axis::Y => vec3(chunk.x, state.pos.y, chunk.z),
        };
        state.update_palette();
    }
}
