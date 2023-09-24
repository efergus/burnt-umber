use std::f32::consts::PI;

use three_d::{
    radians, vec3, Camera, Context, CpuMesh, Cull, DepthTest, Mat4, Program, RenderStates,
    RenderTarget, Vec3, VertexBuffer, degrees,
};

use crate::{
    geometry::{cube_mesh, cylinder_mesh, quad_mesh, tube_mesh, unwrap_mesh},
    to_cylindrical,
};

pub struct Model<'a> {
    positions: &'a VertexBuffer,
    embed: &'a VertexBuffer,
    render_states: RenderStates,
    view: Mat4,
    model: Mat4,
    meta: Mat4,
    tag: u8,
}

pub trait Renderable<T> {
    fn model<'a>(&'a self, state: &T) -> Model<'a>;
}

pub trait Renderer {
    fn render(&mut self, target: &RenderTarget, model: &Model);
}

impl Renderer for Program {
    fn render(&mut self, target: &RenderTarget, model: &Model) {
        target.write(move || {
            self.use_uniform("view", model.view);
            self.use_uniform("model", model.model);
            self.use_uniform("meta", model.meta);
            self.use_uniform_if_required("tag", model.tag as f32);
            assert!(model.positions.vertex_count() == model.embed.vertex_count());
            self.use_vertex_attribute("position", model.positions);
            self.use_vertex_attribute("embed", model.embed);
            self.draw_arrays(
                model.render_states,
                target.viewport(),
                model.positions.vertex_count(),
            );
        });
    }
}

#[derive(PartialEq)]
pub enum Space {
    Linear,
    Cylindrical,
}

pub struct InputState {
    pub pos: Vec3,
    pub saved_pos: Vec3,
    pub cylindrical: Vec3,
    pub saved_cylindrical: Vec3,
    pub chunk: Vec3,
    pub camera: Camera,
    pub space: Space,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera, space: Space) -> Self {
        InputState {
            pos,
            saved_pos: pos,
            cylindrical: to_cylindrical(pos),
            saved_cylindrical: to_cylindrical(pos),
            chunk: vec3(1., 1., 1.),
            camera,
            space,
        }
    }
}

pub struct AxisInput {
    positions: VertexBuffer,
    embed: VertexBuffer,
    axis: u8,
}

impl AxisInput {
    pub fn new(context: &Context, axis: u8) -> AxisInput {
        match axis {
            0 => {
                let tube = tube_mesh(32);
                let tube_wrap = unwrap_mesh(&tube);
                AxisInput {
                    positions: VertexBuffer::new_with_data(context, &tube_wrap),
                    embed: VertexBuffer::new_with_data(context, &tube),
                    axis,
                }
            }
            1 | 2 => AxisInput {
                positions: VertexBuffer::new_with_data(context, &quad_mesh()),
                embed: VertexBuffer::new_with_data(context, &quad_mesh()),
                axis,
            },
            _ => panic!("Unknown axis"),
        }
    }
}

impl Renderable<InputState> for AxisInput {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let pos = state.pos;
        let direction = state.camera.position() - state.camera.target();
        let camera_angle = direction.z.atan2(direction.x);
        let (view, model, meta) = match self.axis {
            0 => (
                Mat4::from_translation(vec3(-0.5, 0.8, 0.0))
                    * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                match state.space {
                    Space::Cylindrical => {
                        Mat4::from_translation(vec3(0.0, pos.y, 0.0))
                            * Mat4::from_angle_y(radians(-camera_angle + PI))
                            * Mat4::from_nonuniform_scale(
                                state.cylindrical.y,
                                0.0,
                                state.cylindrical.y,
                            )
                    }
                    Space::Linear => {
                        Mat4::from_translation(vec3(0.0, pos.y, pos.z))
                            * Mat4::from_nonuniform_scale(1.0, 0.0, 0.0)
                    }
                },
            ),
            1 => (
                Mat4::from_translation(vec3(-1.0, -0.5, 0.0))
                    * Mat4::from_nonuniform_scale(0.2, 1.0, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                match state.space {
                    Space::Cylindrical => {
                        Mat4::from_translation(vec3(0.0, pos.y, 0.0))
                            * Mat4::from_angle_y(radians(-state.cylindrical.x))
                            * Mat4::from_nonuniform_scale(1.0, 0.0, 0.0)
                    }
                    Space::Linear => {
                        Mat4::from_translation(vec3(pos.x, 0.0, pos.z))
                            * Mat4::from_nonuniform_scale(0.0, 1.0, 0.0)
                    }
                },
            ),
            2 => (
                Mat4::from_translation(vec3(-0.5, -1.0, 0.0))
                    * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                match state.space {
                    Space::Cylindrical => {
                        Mat4::from_translation(vec3(pos.x, 0.0, pos.z))
                            * Mat4::from_nonuniform_scale(0.0, 1.0, 0.0)
                    }
                    Space::Linear => {
                        Mat4::from_translation(vec3(pos.x, pos.y, 0.0))
                            * Mat4::from_angle_y(degrees(-90.0))
                            * Mat4::from_nonuniform_scale(1.0, 0.0, 0.0)
                    }
                },
            ),
            _ => panic!("Unknown axis"),
        };
        let embed = if state.space == Space::Cylindrical && self.axis == 0 {
            &self.embed
        } else {
            &self.positions
        };
        Model {
            positions: &self.positions,
            embed,
            render_states: RenderStates::default(),
            tag: self.axis + 1,
            view,
            model,
            meta,
        }
    }
}

pub struct ColorSpace {
    positions: VertexBuffer,
}

impl ColorSpace {
    pub fn cylinder(context: &Context) -> Self {
        ColorSpace {
            positions: VertexBuffer::new_with_data(context, &cylinder_mesh(64)),
        }
    }
    pub fn cube(context: &Context) -> Self {
        ColorSpace {
            positions: VertexBuffer::new_with_data(context, &cube_mesh()),
        }
    }
}

impl Renderable<InputState> for ColorSpace {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = match state.space {
            Space::Linear => {
                Mat4::from_nonuniform_scale(state.chunk.x, state.chunk.y, state.chunk.z)
            }
            Space::Cylindrical => {
                Mat4::from_nonuniform_scale(state.chunk.y, state.chunk.z, state.chunk.y)
            }
        };
        Model {
            positions: &self.positions,
            embed: &self.positions,
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model,
            meta: model,
        }
    }
}

pub struct Cursor {
    positions: VertexBuffer,
}

impl Cursor {
    pub fn cube(context: &Context) -> Self {
        let data = CpuMesh::cube().positions.to_f32();
        Cursor {
            positions: VertexBuffer::new_with_data(context, &data),
        }
    }
}

impl Renderable<InputState> for Cursor {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        Model {
            positions: &self.positions,
            embed: &self.positions,
            render_states: RenderStates {
                depth_test: DepthTest::Less,
                cull: Cull::Back,
                ..Default::default()
            },
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model: Mat4::from_translation(state.pos) * Mat4::from_scale(0.05),
            meta: Mat4::from_scale(0.0),
        }
    }
}
