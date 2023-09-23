use std::f32::consts::PI;

use three_d::{
    context, vec3, Camera, Color, Context, CpuMesh, Cull, DepthTest, Mat4, Program, RenderStates,
    RenderTarget, SquareMatrix, Vec3, VertexBuffer, radians, Zero,
};

use crate::geometry::{cylinder_mesh, quad_mesh, tube_mesh, unwrap_mesh};

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

pub struct InputState {
    pub pos: Vec3,
    pub cylindrical: Vec3,
    pub saved_cylindrical: Vec3,
    pub camera: Camera,
}

impl InputState {
    pub fn new(pos: Vec3, camera: Camera) -> Self {
        InputState { pos, cylindrical: vec3(0.0, 0.0, 0.0), saved_cylindrical: Vec3::zero(), camera }
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
                let tube = tube_mesh(64);
                let tube_wrap = unwrap_mesh(&tube);
                AxisInput {
                    positions: VertexBuffer::new_with_data(context, &tube_wrap),
                    embed: VertexBuffer::new_with_data(context, &tube),
                    axis,
                }
            },
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
        let (view, model, meta) = match self.axis {
            0 => (
                Mat4::from_translation(vec3(-0.5, 0.8, 0.0))
                * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                Mat4::from_translation(vec3(0.0, pos.y, 0.0))
                * Mat4::from_nonuniform_scale(state.cylindrical.y, 0.0, state.cylindrical.y),
            ),
            1 => (
                Mat4::from_translation(vec3(-0.5, -1.0, 0.0))
                * Mat4::from_nonuniform_scale(1.0, 0.2, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                Mat4::from_angle_y(radians(state.cylindrical.x * 2.0 * PI))
                * Mat4::from_translation(vec3(0.0, pos.y, 0.0))
                * Mat4::from_nonuniform_scale(1.0, 0.0, 1.0),
            ),
            2 => (
                Mat4::from_translation(vec3(-1.0, -0.5, 0.0))
                * Mat4::from_nonuniform_scale(0.2, 1.0, 1.0),
                Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                Mat4::from_translation(vec3(pos.x, 0.0, pos.z))
                * Mat4::from_nonuniform_scale(0.0, 1.0, 0.0),
            ),
            _ => panic!("Unknown axis"),
        };
        Model {
            positions: &self.positions,
            embed: &self.embed,
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
    embed: Option<VertexBuffer>,
}

impl ColorSpace {
    pub fn cylinder(context: &Context) -> Self {
        let data = cylinder_mesh(64);
        ColorSpace {
            positions: VertexBuffer::new_with_data(context, &data),
            embed: None,
        }
    }
}

impl Renderable<InputState> for ColorSpace {
    fn model<'a>(&'a self, state: &InputState) -> Model<'a> {
        Model {
            positions: &self.positions,
            embed: (self.embed.as_ref()).unwrap_or(&self.positions),
            render_states: RenderStates::default(),
            tag: 7,
            view: state.camera.projection() * state.camera.view(),
            model: Mat4::identity(),
            meta: Mat4::identity(),
        }
    }
}

pub struct Cursor {
    positions: VertexBuffer,
}

impl Cursor {
    pub fn cube(context: &Context) -> Self {
        let data = &CpuMesh::cube().positions.to_f32();
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
            model: Mat4::from_translation(state.pos) * Mat4::from_scale(0.2),
            meta: Mat4::from_translation(state.pos) * Mat4::from_scale(0.5),
        }
    }
}
