use std::rc::Rc;

use cgmath::{vec3, Zero, InnerSpace, SquareMatrix, ElementWise};
use three_d::{RenderStates, Vec3, Context, Mat4};

use crate::{mesh::Mesh, embed::Embedding, pre_embed::plane, input::InputState};

use super::{ColorElement, ColorModel, SpaceModel};


#[derive(Clone, Copy)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
pub struct ColorAxis {
    pub positions: Mesh,
    pub input: Mesh,
    pub embed: Mesh,
    pub axis: Axis,
    color_embedding: Rc<dyn Embedding<Vec3>>,
    unit_x: Vec3,
    unit_y: Vec3,
    pos: Vec3,
}

impl ColorAxis {
    pub fn new(context: &Context, axis: Axis, color_embedding: Rc<dyn Embedding<Vec3>>,) -> ColorAxis {
        let unit_x = match axis {
            Axis::X | Axis::Y => vec3(1.0, 0.0, 0.0),
            Axis::Z => vec3(0.0, 0.0, 1.0),
        };
        let unit_y = vec3(0.0, 1.0, 0.0);
        let (horizontal_subdivisions, vertical_subdivisions) = match axis {
            Axis::X | Axis::Z => (16, 1),
            Axis::Y => (1, 16),
        };

        let mesh = Mesh::new(context, plane(horizontal_subdivisions, vertical_subdivisions, unit_x, unit_y, vec3(0.0, 0.0, 0.0)));
        let positions = Mesh::from_mesh_embedded(context, &mesh, |pos| vec3(pos.dot(unit_x), pos.dot(unit_y), 0.0));

        ColorAxis {
            positions,
            input: Mesh::from_mesh(context, &mesh),
            embed: Mesh::from_mesh(context, &mesh),
            axis,
            color_embedding,
            unit_x,
            unit_y,
            pos: Vec3::zero(),
        }
    }

    // pub fn update<F>(&mut self, position: Vec3, embedding: F)
    // where
    //     F: Fn(Vec3) -> Vec3,
    // {
    //     match self.axis {
    //         Axis::X => self.embed.embed_from(&self.input, |pos: Vec3| {
    //             embedding(vec3(pos.x, position.y, position.z))
    //         }),
    //         Axis::Y => self.embed.embed_from(&self.input, |pos: Vec3| {
    //             embedding(vec3(position.x, pos.y, position.z))
    //         }),
    //         Axis::Z => self.embed.embed_from(&self.input, |pos: Vec3| {
    //             embedding(vec3(position.x, position.y, pos.z))
    //         }),
    //     }
    // }

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

    // fn model_matrix(&self, _state: &InputState) -> Mat4 {
    //     Mat4::identity()
    // }

    // fn meta_matrix(&self, _state: &InputState) -> Mat4 {
    //     Mat4::identity()
    // }

    // pub fn scene_model<'a>(&'a self, state: &InputState) -> Model<'a> {
    //     // let pos = state.pos;
    //     // let direction = state.camera.position() - state.camera.target();
    //     // let camera_angle = direction.z.atan2(direction.x);
    //     Model {
    //         positions: &self.positions.vertex_buffer(),
    //         embed: &self.embed.vertex_buffer(),
    //         indices: &self.positions.element_buffer(),
    //         render_states: RenderStates::default(),
    //         tag: self.axis as u8 + 1,
    //         view: self.view_matrix(state),
    //         model: self.model_matrix(state),
    //         meta: self.meta_matrix(state),
    //     }
    // }
}

impl ColorElement<InputState> for ColorAxis {
    fn color_model(&self) -> super::ColorModel {
        ColorModel {
            positions: self.positions.vertex_buffer(),
            embed: self.embed.vertex_buffer(),
            indices: self.positions.element_buffer(),
            render_states: RenderStates::default(),
            view: self.view_matrix(),
            model: Mat4::identity(),
            meta: Mat4::identity(),
        }
    }

    fn invert_space(&self, pos: Vec3) -> Vec3 {
        match self.axis {
            Axis::X => vec3(pos.x, self.pos.y, self.pos.z),
            Axis::Y => vec3(self.pos.x, pos.y, self.pos.z),
            Axis::Z => vec3(self.pos.x, self.pos.y, pos.x),
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
            Axis::X => vec3(0.0, pos.y, pos.z),
            Axis::Y => vec3(pos.x, 0.0, pos.z),
            Axis::Z => vec3(pos.x, pos.y, 0.0),
        };

        self.embed.embed_from_positions(self.input.positions(), |pos| {
            self.color_embedding.embed(pos.mul_element_wise(scale) + origin)
        })
    }

    fn space_model(&self) -> super::SpaceModel {
        SpaceModel {
            positions: self.positions.vertex_buffer(),
            embed: self.input.vertex_buffer(),
            indices: self.positions.element_buffer(),
            render_states: RenderStates::default(),
            view: self.view_matrix(),
            model: Mat4::identity(),
            meta: Mat4::identity(),
        }
    }
}