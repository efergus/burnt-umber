use std::rc::Rc;

use cgmath::vec3;
use three_d::{Context, Program, RenderTarget, ScissorBox, Vec3};

use crate::{
    element::{
        coloraxis::{Axis, ColorAxis},
        colorchips::ColorChips,
        colorspace::ColorSpace,
        embedswitcher::EmbedSwitcher,
        ColorElement, ModelGraph, TaggedColorModel,
    },
    embed::{CylindricalEmbedding, Embedding, OkhsvEmbedding},
    pre_embed,
    renders::{Cursor, CursorState, Renderable},
    InputState, Renderer,
};

pub struct Target<'a> {
    pub target: &'a RenderTarget<'a>,
    pub program: &'a mut Program,
    pub pos_target: &'a RenderTarget<'a>,
    pub pos_program: &'a mut Program,
}

pub trait Scene<T> {
    fn update(&mut self, state: &T);
    fn render(&mut self, target: &mut Target, state: &mut T);
}

pub struct ColorScene {
    cursor: Cursor,
    elements: Vec<Box<dyn ColorElement<InputState>>>,
    prev_tag: u8,
}

impl ColorScene {
    pub fn new(context: &Context) -> Self {
        let space = pre_embed::cube(48, 6, 2);
        let space = ColorSpace::new(context, space);
        Self {
            cursor: Cursor::cube(&context),
            elements: vec![
                Box::new(space),
                Box::new(ColorAxis::new(&context, Axis::X)),
                Box::new(ColorAxis::new(&context, Axis::Y)),
                Box::new(ColorAxis::new(&context, Axis::Z)),
                Box::new(ColorChips::new(&context, 6, 0.2)),
                Box::new(EmbedSwitcher::new(&context, true, 0.0)),
                Box::new(EmbedSwitcher::new(&context, false, 0.25)),
            ],
            prev_tag: 0,
        }
    }

    pub fn render_graph(&self, target: &mut Target, graph: &ModelGraph, tag: u16) {
        match graph {
            ModelGraph::Color(model) => {
                target.program.render(target.target, model);
            }
            ModelGraph::Space(model) => {
                let tagged_model = TaggedColorModel { model: model, tag };
                target.pos_program.render(target.pos_target, &tagged_model);
            }
            ModelGraph::Vec(models) => {
                for model in models {
                    self.render_graph(target, model, tag);
                }
            }
        }
    }
}

impl Scene<InputState> for ColorScene {
    fn update(&mut self, state: &InputState) {
        for element in &mut self.elements {
            element.update(state);
        }
    }

    fn render(&mut self, target: &mut Target, state: &mut InputState) {
        let screen = target.target;
        for (tag, element) in self.elements.iter().enumerate() {
            let model_graph = element.model();
            self.render_graph(target, &model_graph, (tag + 1) as u16);
        }
        target.program.render(
            screen,
            &self.cursor.model(&CursorState {
                pos: state.space_embedding.embed(state.pos),
                view: state.camera.projection() * state.camera.view(),
            }),
        );

        let scissor_box = ScissorBox {
            x: state.mouse_pos.x as i32,
            y: (target.pos_target.height() as i32) - (state.mouse_pos.y as i32),
            width: 1,
            height: 1,
        };
        let pos = target
            .pos_target
            .read_color_partially::<[f32; 4]>(scissor_box)[0];
        let tag = pos[3] as u8;
        let pos = vec3(pos[0], pos[1], pos[2]);
        if tag != self.prev_tag {
            if self.prev_tag > 0 {
                self.elements[self.prev_tag as usize - 1].exited();
            }
            if tag > 0 {
                self.elements[tag as usize - 1].entered();
            }
            self.prev_tag = tag;
        }
        if tag > 0 {
            if state.press {
                self.elements[tag as usize - 1].clicked();
            }
            if let Some(pos) = self.elements[tag as usize - 1].invert_space(pos) {
                state.pos = pos;
            }
            state.color = state.color_embedding.embed(pos);
            self.elements[tag as usize - 1].update_state(state);
        } else {
            state.pos = state.saved_pos;
        }
    }
}
