use std::rc::Rc;

use cgmath::vec3;
use three_d::{Context, Program, RenderTarget, ScissorBox, Vec3};

use crate::{
    element::{
        coloraxis::{Axis, ColorAxis},
        colorchips::ColorChips,
        colorspace::ColorSpace,
        ColorElement, ModelGraph, TaggedColorModel,
    },
    embed::{CylindricalEmbedding, Embedding, OkhsvEmbedding},
    pre_embed,
    renders::{ColorChip, Cursor, CursorState, Renderable},
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
    color_embedding: Rc<dyn Embedding<Vec3>>,
    space_embedding: Rc<dyn Embedding<Vec3>>,
}

impl ColorScene {
    fn new<T: Embedding + 'static, U: Embedding + 'static>(
        context: &Context,
        space_embedding: T,
        color_embedding: U,
    ) -> Self {
        let space = pre_embed::cube(48, 6, 2);
        let space_embedding = Rc::new(space_embedding);
        let color_embedding = Rc::new(color_embedding);
        let space = ColorSpace::new(
            context,
            space,
            space_embedding.clone(),
            color_embedding.clone(),
        );
        Self {
            cursor: Cursor::cube(&context),
            elements: vec![
                Box::new(space),
                Box::new(ColorAxis::new(&context, Axis::X, color_embedding.clone())),
                Box::new(ColorAxis::new(&context, Axis::Y, color_embedding.clone())),
                Box::new(ColorAxis::new(&context, Axis::Z, color_embedding.clone())),
                Box::new(ColorChips::new(&context, 6, 0.2, color_embedding.clone())),
            ],
            color_embedding,
            space_embedding,
        }
    }

    pub fn cylinder(context: &Context) -> Self {
        // let space_embedding = ComposedEmbedding::new(
        //     Box::new(SwapAxesEmbedding::new(Axis::X, Axis::Y)),
        //     Box::new(LinSrgbOklabEmbedding {}));
        // Self::new(context, space_embedding, LinSrgbOklabEmbedding {})
        Self::new(context, CylindricalEmbedding {}, OkhsvEmbedding {})
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
                pos: self.space_embedding.embed(state.pos),
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
        if tag > 0 {
            state.pos = self.elements[tag as usize - 1].invert_space(pos);
            state.color = self.color_embedding.embed(pos);
            self.elements[tag as usize - 1].update_state(state);
        } else {
            state.pos = state.saved_pos;
            state.update_palette();
        }
    }
}
