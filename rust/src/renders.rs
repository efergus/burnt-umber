use std::f32::consts::PI;

use palette::{okhsv, FromColor, LinSrgb, Oklab};
use three_d::{
    degrees, radians, vec2, vec3, Camera, Context, Cull, DepthTest, ElementBuffer, InnerSpace,
    Mat4, Program, RenderStates, RenderTarget, SquareMatrix, Vec3, VertexBuffer,
};

use crate::{
    geometry::{cube_mesh, cylinder_mesh, quad_mesh, subdivide_n, tube_mesh, unwrap_mesh},
    mesh::Mesh,
    to_cylindrical,
};

pub struct Model<'a> {
    positions: &'a VertexBuffer,
    embed: &'a VertexBuffer,
    indices: &'a ElementBuffer,
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
            self.draw_elements(
                model.render_states,
                target.viewport(),
                model.indices,
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
    pub chunk: Vec3,
    pub camera: Camera,
    pub input: bool,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState {
            pos,
            saved_pos: pos,
            chunk: vec3(1., 1., 1.),
            camera,
            input: false,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

pub struct AxisInput {
    pub positions: Mesh,
    pub input: Mesh,
    pub embed: Mesh,
    pub axis: Axis,
}

impl AxisInput {
    pub fn new(context: &Context, axis: Axis) -> AxisInput {
        match axis {
            Axis::X => {
                let tube = tube_mesh(32);
                let tube_wrap = unwrap_mesh(&tube);
                AxisInput {
                    positions: Mesh::from_positions(context, tube_wrap.clone()),
                    input: Mesh::from_positions(context, tube_wrap),
                    embed: Mesh::from_positions(context, tube),
                    axis,
                }
            }
            Axis::Y => {
                let line: Vec<Vec3> = quad_mesh()
                    .iter()
                    .map(|pos| vec3(0.0, pos.y, 0.0))
                    .collect();
                AxisInput {
                    positions: Mesh::from_positions(context, quad_mesh()),
                    input: Mesh::from_positions(context, line.clone()),
                    embed: Mesh::from_positions(context, line),
                    axis,
                }
            }
            Axis::Z => {
                let line = quad_mesh()
                    .iter()
                    .map(|pos| vec3(0.0, 0.0, pos.x))
                    .collect();
                AxisInput {
                    positions: Mesh::from_positions(context, quad_mesh()),
                    input: Mesh::from_positions(context, line),
                    embed: Mesh::from_positions(context, quad_mesh()),
                    axis,
                }
            }
            _ => panic!("Unknown axis"),
        }
    }

    pub fn update<F>(&mut self, position: Vec3, embedding: F)
    where
        F: Fn(Vec3) -> Vec3,
    {
        match self.axis {
            Axis::X => self.input.embed(|pos| vec3(pos.x, position.y, position.z)),
            Axis::Y => self.input.embed(|pos| vec3(position.x, pos.y, position.z)),
            Axis::Z => self.input.embed(|pos| vec3(position.x, position.y, pos.z)),
        }
        self.embed.embed_from(&self.input, embedding);
    }

    fn view_matrix(&self, _state: &InputState) -> Mat4 {
        // let pos = state.pos;
        // let direction = state.camera.position() - state.camera.target();
        // let camera_angle = direction.z.atan2(direction.x);
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

    fn model_matrix(&self, _state: &InputState) -> Mat4 {
        Mat4::identity()
    }

    fn meta_matrix(&self, _state: &InputState) -> Mat4 {
        Mat4::identity()
    }

    pub fn scene_model<'a>(&'a self, state: &InputState) -> Model<'a> {
        // let pos = state.pos;
        // let direction = state.camera.position() - state.camera.target();
        // let camera_angle = direction.z.atan2(direction.x);
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.embed.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: self.axis as u8 + 1,
            view: self.view_matrix(state),
            model: self.model_matrix(state),
            meta: self.meta_matrix(state),
        }
    }

    pub fn input_model<'a>(&'a self, state: &InputState) -> Model<'a> {
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.input.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates {
                depth_test: DepthTest::Less,
                cull: Cull::Back,
                ..Default::default()
            },
            tag: self.axis as u8 + 1,
            view: self.view_matrix(state),
            model: Mat4::identity(),
            meta: Mat4::identity(),
        }
    }
}

impl Renderable<InputState> for AxisInput {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        self.scene_model(state)
    }
}

pub struct ColorChip {
    positions: Mesh,
}

impl ColorChip {
    pub fn new(context: &Context) -> Self {
        ColorChip {
            positions: Mesh::from_positions(context, quad_mesh()),
        }
    }
}

impl Renderable<InputState> for ColorChip {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let pos = state.pos;
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.positions.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: Mat4::identity(),
            model: Mat4::from_translation(vec3(0.8, 0.8, 0.0)) * Mat4::from_scale(0.2),
            meta: Mat4::from_translation(pos) * Mat4::from_scale(0.0),
        }
    }
}

pub fn okhsv_embed(pos: Vec3) -> Vec3 {
    let flat = vec2(pos.x, pos.z);
    let angle = -flat.y.atan2(flat.x) / std::f32::consts::PI / 2.0;
    let hsv = okhsv::Okhsv::new(angle * 360.0, flat.magnitude(), pos.y);
    let oklab = Oklab::from_color(hsv);
    let rgb = LinSrgb::from_color(oklab);
    vec3(rgb.red, rgb.green, rgb.blue)
}

pub fn okhsv_embed_oklab(pos: Vec3) -> Vec3 {
    let flat = vec2(pos.x, pos.z);
    let angle = -flat.y.atan2(flat.x) / std::f32::consts::PI / 2.0;
    let hsv = okhsv::Okhsv::new(angle * 360.0, flat.magnitude(), pos.y);
    let oklab = Oklab::from_color(hsv);
    vec3(oklab.l, oklab.a, oklab.b)
}

fn siny_embed(pos: Vec3) -> Vec3 {
    vec3(pos.x, (pos.y * PI).sin(), pos.z)
}

pub struct ColorSpace {
    positions: Mesh,
    embedding: Mesh,
}

impl ColorSpace {
    pub fn cylinder(context: &Context) -> Self {
        let m = cylinder_mesh(64);
        let m = subdivide_n(&m, 1);
        let positions = Mesh::from_positions(context, m.clone());
        let embedding = Mesh::from_positions(context, m);

        ColorSpace {
            positions,
            embedding,
        }
    }
    pub fn cube(context: &Context) -> Self {
        ColorSpace {
            positions: Mesh::from_positions(context, cube_mesh()),
            embedding: Mesh::from_positions(context, cube_mesh()),
        }
    }
}

impl ColorSpace {
    pub fn okhsv_embed(&mut self, chunk: Vec3) {
        use cgmath::ElementWise;
        self.embedding
            .embed_from(&self.positions, |pos| okhsv_embed(pos.mul_element_wise(chunk)));
    }

    pub fn okhsv_embed_oklab(&mut self, chunk: Vec3) {
        use cgmath::ElementWise;
        self.embedding
            .embed_from(&self.positions, |pos| okhsv_embed_oklab(pos.mul_element_wise(chunk)));
    }

    pub fn input_model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = Mat4::from_nonuniform_scale(state.chunk.x, state.chunk.y, state.chunk.z);
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.positions.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model,
            meta: Mat4::identity(),
        }
    }
}

impl Renderable<InputState> for ColorSpace {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = Mat4::from_nonuniform_scale(state.chunk.z, state.chunk.y, state.chunk.z);
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.embedding.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model,
            meta: Mat4::identity(),
        }
    }
}

pub struct Cursor {
    positions: Mesh,
}

impl Cursor {
    pub fn cube(context: &Context) -> Self {
        let data = three_d::CpuMesh::cube().positions.to_f32();
        Cursor {
            positions: Mesh::from_positions(context, data),
        }
    }
}

impl Renderable<InputState> for Cursor {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.positions.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model: Mat4::from_translation(state.pos) * Mat4::from_scale(0.05),
            meta: Mat4::from_scale(0.0),
        }
    }
}
