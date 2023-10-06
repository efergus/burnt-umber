use std::f32::consts::PI;

use palette::{okhsv, Oklab, FromColor, oklab, LinSrgb};
use three_d::{
    degrees, radians, vec3, Camera, Context, CpuMesh, Cull, DepthTest, Mat4, Program, RenderStates,
    RenderTarget, Vec3, VertexBuffer, SquareMatrix, vec2, InnerSpace, ElementBuffer,
};

use crate::{
    geometry::{cube_mesh, cylinder_mesh, quad_mesh, tube_mesh, unwrap_mesh, subdivide, subdivide_n},
    to_cylindrical,
};

pub struct Mesh {
    pub cpu_positions: Vec<Vec3>,
    pub cpu_indices: Option<Vec<u32>>,
    pub positions: VertexBuffer,
    pub indices: ElementBuffer,
}

impl Mesh {
    pub fn new(context: &Context, cpu_positions: Vec<Vec3>, cpu_indices: Option<Vec<u32>>) -> Self {
        let positions = VertexBuffer::new_with_data(context, &cpu_positions);
        let indices = match &cpu_indices {
            Some(cpu_indices) => ElementBuffer::new_with_data(context, &cpu_indices),
            None => ElementBuffer::new_with_data(context, &Vec::from_iter(0..cpu_positions.len() as u32)),
        };
        Mesh {
            cpu_positions,
            cpu_indices,
            positions,
            indices,
        }
    }

    pub fn embed<T: Fn(&Vec3) -> Vec3>(&mut self, embedding: T) {
        let cpu_positions: Vec<Vec3> = self
            .cpu_positions
            .iter()
            .map(|pos| embedding(pos))
            .collect();
        self.positions.fill(&cpu_positions);
    }
}

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
                                state.cylindrical.z,
                                0.0,
                                state.cylindrical.z,
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
                        Mat4::from_translation(vec3(pos.x, 0.0, pos.z))
                            * Mat4::from_nonuniform_scale(0.0, 1.0, 0.0)
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
                        Mat4::from_translation(vec3(0.0, pos.y, 0.0))
                            * Mat4::from_angle_y(radians(-state.cylindrical.x))
                            * Mat4::from_nonuniform_scale(1.0, 0.0, 0.0)
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

pub struct ColorChip {
    positions: VertexBuffer,    
}


impl ColorChip {
    pub fn new(context: &Context) -> Self {
        ColorChip {
            positions: VertexBuffer::new_with_data(context, &quad_mesh()),
        }
    }
}

impl Renderable<InputState> for ColorChip {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let pos = state.pos;
        Model {
            positions: &self.positions,
            embed: &self.positions,
            render_states: RenderStates::default(),
            tag: 7,
            view: Mat4::identity(),
            model: Mat4::from_translation(vec3(0.8, 0.8, 0.0)) * Mat4::from_scale(0.2),
            meta: Mat4::from_translation(pos) * Mat4::from_scale(0.0),
        }
    }
}

fn okhsv_embed(pos: Vec3) -> Vec3{
    let flat = vec2(pos.x, pos.z);
    let angle = -flat.y.atan2(flat.x) / std::f32::consts::PI / 2.0;
    let hsv = okhsv::Okhsv::new(angle * 360.0, flat.magnitude(), pos.y);
    let oklab = Oklab::from_color(hsv);
    let rgb = LinSrgb::from_color(oklab);
    vec3(rgb.red, rgb.green, rgb.blue)
}

fn siny_embed(pos: Vec3) -> Vec3{
    vec3(pos.x, (pos.y*PI).sin(), pos.z)
}

pub struct ColorSpace {
    positions: VertexBuffer,
    embedding: Mesh,
}

impl ColorSpace {
    pub fn cylinder(context: &Context) -> Self {
        let m = cylinder_mesh(64);
        let m = subdivide_n(&m, 4);

        ColorSpace {
            positions: VertexBuffer::new_with_data(context, &m),
            embedding: Mesh::new(context, m, None),
        }
    }
    pub fn cube(context: &Context) -> Self {
        ColorSpace {
            positions: VertexBuffer::new_with_data(context, &cube_mesh()),
            embedding: Mesh::new(context, cube_mesh(), None),
        }
    }
}

impl ColorSpace {
    pub fn okhsv_embed(&mut self, chunk: Vec3) {
        use cgmath::ElementWise;
        self.embedding.embed(|pos| okhsv_embed(pos.mul_element_wise(chunk)));
    }
}

impl Renderable<InputState> for ColorSpace {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = match state.space {
            Space::Linear => {
                Mat4::from_nonuniform_scale(state.chunk.x, state.chunk.y, state.chunk.z)
            }
            Space::Cylindrical => {
                Mat4::from_nonuniform_scale(state.chunk.z, state.chunk.y, state.chunk.z)
            }
        };
        Model {
            positions: &self.positions,
            embed: &self.embedding.positions,
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model,
            meta: Mat4::identity(),
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
            model: Mat4::from_translation(okhsv_embed(state.pos)) * Mat4::from_scale(0.05),
            meta: Mat4::from_scale(0.0),
        }
    }
}
