use std::{f32::consts::PI, rc::Rc};

use palette::{okhsv, FromColor, Oklab};
use three_d::{
    vec3, Camera, Context, ElementBuffer, Mat4, Program, RenderStates, RenderTarget, SquareMatrix,
    Vec3, VertexBuffer,
};

use crate::{
    embed::Embedding,
    from_cylindrical,
    geometry::{max3, quad_mesh, tube_mesh, unwrap_mesh},
    mesh::{CpuMesh, Mesh},
    pre_embed,
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
            self.draw_elements(model.render_states, target.viewport(), model.indices);
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
        }
    }

    pub fn update<F>(&mut self, position: Vec3, embedding: F)
    where
        F: Fn(Vec3) -> Vec3,
    {
        match self.axis {
            Axis::X => self.embed.embed_from(&self.input, |pos: Vec3| {
                embedding(vec3(pos.x, position.y, position.z))
            }),
            Axis::Y => self.embed.embed_from(&self.input, |pos: Vec3| {
                embedding(vec3(position.x, pos.y, position.z))
            }),
            Axis::Z => self.embed.embed_from(&self.input, |pos: Vec3| {
                embedding(vec3(position.x, position.y, pos.z))
            }),
        }
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

impl Renderable<Vec3> for ColorChip {
    fn model<'a>(&'a self, pos: &Vec3) -> Model<'a> {
        Model {
            positions: &self.positions.vertex_buffer(),
            embed: &self.positions.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: Mat4::identity(),
            model: Mat4::from_translation(vec3(0.8, 0.8, 0.0)) * Mat4::from_scale(0.2),
            meta: Mat4::from_translation(*pos) * Mat4::from_scale(0.0),
        }
    }
}

pub fn okhsv_embed_oklab(pos: Vec3) -> Vec3 {
    let hsv = okhsv::Okhsv::new(pos.x * 360.0, pos.z, pos.y);
    let oklab = Oklab::from_color(hsv);
    vec3(oklab.l, oklab.a, oklab.b)
}

pub struct ColorSpace {
    input: Mesh,
    space: Mesh,
    color: Mesh,
    space_embedding: Rc<dyn Embedding<Vec3>>,
    color_embedding: Rc<dyn Embedding<Vec3>>,
    prev_chunk: Vec3,
}

impl ColorSpace {
    pub fn new(
        context: &Context,
        mesh: CpuMesh,
        space_embedding: Rc<dyn Embedding<Vec3>>,
        color_embedding: Rc<dyn Embedding<Vec3>>,
    ) -> Self {
        // let m = pre_embed::cylinder(16, 6, 4);
        let split = mesh.split_triangles();
        let input = Mesh::new(context, split.clone());
        let positions = Mesh::from_mesh_embedded(context, &input, |pos| space_embedding.embed(pos));
        let embeded = Mesh::from_mesh_embedded(context, &input, |pos| color_embedding.embed(pos));

        ColorSpace {
            space: positions,
            input,
            color: embeded,
            space_embedding,
            color_embedding,
            prev_chunk: vec3(1.0, 1.0, 1.0),
        }
    }
}

impl ColorSpace {
    pub fn update(&mut self, chunk: Vec3) {
        use cgmath::ElementWise;
        // self.color.embed_from_triangles(&self.input, |pos| {
        //     let pos = okhsv_embed_oklab(
        //         vec3(
        //             max3(vec3(pos[0].x, pos[1].x, pos[2].x)),
        //             max3(vec3(pos[0].y, pos[1].y, pos[2].y)),
        //             max3(vec3(pos[0].z, pos[1].z, pos[2].z)),
        //         )
        //         .mul_element_wise(chunk),
        //     );
        //     [pos, pos, pos]
        // });
        if self.prev_chunk != chunk {
            self.color
                .embed_from_positions(self.input.positions(), |pos| {
                    self.color_embedding.embed(pos.mul_element_wise(chunk))
                });
                self.prev_chunk = chunk;
            }
    }
    pub fn space_model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = Mat4::from_nonuniform_scale(state.chunk.z, state.chunk.y, state.chunk.z);
        Model {
            positions: &self.space.vertex_buffer(),
            embed: &self.input.vertex_buffer(),
            indices: &self.space.element_buffer(),
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model,
            meta: model,
        }
    }
}

impl Renderable<InputState> for ColorSpace {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        let model = Mat4::from_nonuniform_scale(state.chunk.z, state.chunk.y, state.chunk.z);
        Model {
            positions: &self.space.vertex_buffer(),
            embed: &self.color.vertex_buffer(),
            indices: &self.space.element_buffer(),
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
            model: Mat4::from_translation(from_cylindrical(state.pos)) * Mat4::from_scale(0.05),
            meta: Mat4::from_scale(0.0),
        }
    }
}
