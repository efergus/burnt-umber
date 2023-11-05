use cgmath::{vec2, vec3, ElementWise, InnerSpace, SquareMatrix, Zero};
use three_d::{degrees, radians, Context, Mat4, RenderStates, Vec3};

use crate::{
    embed::{AxisRepresentation, CylindricalEmbedding, StaticEmbedding},
    input::InputState,
    mesh::Mesh,
    pre_embed::plane,
};

use super::{ColorElement, ColorModel, ModelGraph};

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
pub struct ColorAxis {
    pub positions: Mesh,
    pub cursor_positions: Mesh,
    pub input: Mesh,
    pub embed: Mesh,
    pub axis: Axis,
    camera_view: Mat4,
    pos: Vec3,
    color: Vec3,
    space: Vec3,
    hover: bool,
    representation: AxisRepresentation,
}

impl ColorAxis {
    pub fn new(context: &Context, axis: Axis) -> ColorAxis {
        let unit_x = match axis {
            Axis::X | Axis::Y => vec3(1.0, 0.0, 0.0),
            Axis::Z => vec3(0.0, 0.0, 1.0),
        };
        let unit_y = vec3(0.0, 1.0, 0.0);
        let (horizontal_subdivisions, vertical_subdivisions) = match axis {
            Axis::X => (32, 1),
            Axis::Y => (1, 4),
            Axis::Z => (16, 1),
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

        let mut cursor_positions = Mesh::new(
            context,
            plane(8, 1, Vec3::unit_x(), Vec3::unit_y(), vec3(0.0, 0.0, -0.02)),
        );
        cursor_positions.embed(CylindricalEmbedding::static_embed);

        ColorAxis {
            positions,
            cursor_positions,
            input: Mesh::from_mesh(context, &mesh),
            embed: Mesh::from_mesh(context, &mesh),
            axis,
            camera_view: Mat4::identity(),
            pos: Vec3::zero(),
            color: Vec3::zero(),
            space: Vec3::zero(),
            hover: false,
            representation: AxisRepresentation::Cylindrical,
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
        let pos = self.space;
        let flatten = Mat4::from_angle_z(degrees(-90.0));
        let (cursor_model, cursor_meta) = match self.representation {
            AxisRepresentation::Cylindrical => {
                let angle = pos.z.atan2(pos.x);
                let size = vec2(pos.x, pos.z).magnitude();
                let lift = Mat4::from_translation(vec3(0.0, pos.y, 0.0));
                let shift = Mat4::from_translation(vec3(0.0, 0.0, size));
                let rotation = Mat4::from_angle_y(radians(-angle));
                let scale_y = Mat4::from_nonuniform_scale(1.0, pos.y, 1.0);
                let scale_z = Mat4::from_nonuniform_scale(1.0, size, 1.0);
                let cursor_model = match self.axis {
                    Axis::X => Mat4::identity(),
                    Axis::Y => Mat4::from_angle_y(degrees(90.0)) * rotation * shift * scale_y,
                    Axis::Z => lift * rotation * flatten * scale_z,
                };
                let cursor_meta = match self.axis {
                    Axis::Y | Axis::Z => Mat4::from_translation(self.color) * Mat4::from_scale(0.0),
                    Axis::X => Mat4::from_translation(vec3(0.8, 0.0, 0.0)) * Mat4::from_scale(0.0),
                };
                (cursor_model, cursor_meta)
            }
            AxisRepresentation::Linear => {
                let cursor_model = match self.axis {
                    Axis::X => Mat4::from_translation(vec3(0.0, pos.y, pos.z)) * flatten,
                    Axis::Y => Mat4::from_translation(vec3(pos.x, 0.0, pos.z)),
                    Axis::Z => {
                        Mat4::from_translation(vec3(pos.x, pos.y, 0.0))
                            * Mat4::from_angle_x(degrees(90.0))
                    }
                };
                let cursor_meta = Mat4::from_translation(self.color) * Mat4::from_scale(0.0);
                (cursor_model, cursor_meta)
            }
        };
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
            ModelGraph::Color(ColorModel {
                positions: self.cursor_positions.vertex_buffer(),
                embed: self.cursor_positions.vertex_buffer(),
                indices: self.cursor_positions.element_buffer(),
                render_states: RenderStates {
                    depth_test: three_d::DepthTest::Always,
                    ..Default::default()
                },
                view: self.camera_view,
                model: cursor_model,
                meta: cursor_meta,
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

    fn entered(&mut self) {
        self.hover = true;
    }

    fn exited(&mut self) {
        self.hover = false;
    }

    fn invert_space(&self, pos: Vec3) -> Option<Vec3> {
        Some(match self.axis {
            Axis::X => vec3(pos.x + self.pos.x - 0.5, self.pos.y, self.pos.z),
            Axis::Y => vec3(self.pos.x, pos.y, self.pos.z),
            Axis::Z => vec3(self.pos.x, self.pos.y, pos.z),
        })
    }

    fn update(&mut self, state: &InputState) {
        let pos = state.pos;
        if !self.hover {
            self.pos = pos;
        }
        self.color = state.color_embedding.embed(pos);
        self.space = state.space_embedding.embed(pos);
        self.representation = state.space_embedding.axis_representation();
        let scale = match self.axis {
            Axis::X | Axis::Z => vec3(1.0, 0.0, 1.0),
            Axis::Y => vec3(0.0, 1.0, 0.0),
        };

        let origin = match self.axis {
            Axis::X => vec3(self.pos.x - 0.5, pos.y, pos.z),
            Axis::Y => vec3(pos.x, 0.0, pos.z),
            Axis::Z => vec3(pos.x, pos.y, 0.0),
        };

        self.embed
            .embed_from_positions(self.input.positions(), |pos| {
                state
                    .color_embedding
                    .embed(pos.mul_element_wise(scale) + origin)
            });

        self.camera_view = state.camera.projection() * state.camera.view();
    }

    fn update_state(&self, state: &mut InputState) {
        let chunk = state.chunk;
        state.chunk = match self.axis {
            Axis::X | Axis::Z => chunk,
            Axis::Y => vec3(chunk.x, state.pos.y, chunk.z),
        };
    }
}
