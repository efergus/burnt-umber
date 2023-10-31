use three_d::{Context, Mat4, Program, RenderStates, RenderTarget, Vec3};

use crate::{
    element::{ColorModel, TaggedColorModel},
    mesh::Mesh,
};

pub trait Renderable<'a, T, M> {
    fn model(&'a self, state: &T) -> M;
}

pub trait Renderer<T> {
    fn render(&mut self, target: &RenderTarget, model: &T);
}

impl<'a> Renderer<ColorModel<'a>> for Program {
    fn render(&mut self, target: &RenderTarget, model: &ColorModel) {
        target.write(move || {
            self.use_uniform("view", model.view);
            self.use_uniform("model", model.model);
            self.use_uniform("meta", model.meta);
            self.use_vertex_attribute("position", model.positions);
            self.use_vertex_attribute("embed", model.embed);
            self.draw_elements(model.render_states, target.viewport(), model.indices);
        });
    }
}

impl<'a> Renderer<TaggedColorModel<'a>> for Program {
    fn render(&mut self, target: &RenderTarget, model: &TaggedColorModel<'a>) {
        target.write(move || {
            self.use_uniform("view", model.model.view);
            self.use_uniform("model", model.model.model);
            self.use_uniform("meta", model.model.meta);
            self.use_uniform_if_required("tag", model.tag as f32);
            self.use_vertex_attribute("position", model.model.positions);
            self.use_vertex_attribute("embed", model.model.embed);
            self.draw_elements(
                model.model.render_states,
                target.viewport(),
                model.model.indices,
            );
        });
    }
}

#[derive(PartialEq)]
pub enum Space {
    Linear,
    Cylindrical,
}
pub struct Cursor {
    positions: Mesh,
}

pub struct CursorState {
    pub pos: Vec3,
    pub view: Mat4,
}

impl Cursor {
    pub fn cube(context: &Context) -> Self {
        let data = three_d::CpuMesh::cube().positions.to_f32();
        Cursor {
            positions: Mesh::from_positions(context, data),
        }
    }
}

impl<'a> Renderable<'a, CursorState, ColorModel<'a>> for Cursor {
    fn model(&'a self, state: &CursorState) -> ColorModel<'a> {
        ColorModel {
            positions: &self.positions.vertex_buffer(),
            embed: &self.positions.vertex_buffer(),
            indices: &self.positions.element_buffer(),
            render_states: RenderStates {
                depth_test: three_d::DepthTest::Always,
                ..Default::default()
            },
            view: state.view,
            model: Mat4::from_translation(state.pos) * Mat4::from_scale(0.05),
            meta: Mat4::from_scale(0.0),
        }
    }
}
