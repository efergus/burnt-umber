use std::rc::Rc;

use three_d::{Context, Program, RenderTarget};

use crate::{
    element::{
        coloraxis::{Axis, ColorAxis},
        colorspace::ColorSpace,
        ColorElement, ModelGraph, TaggedColorModel,
    },
    embed::{CylindricalEmbedding, Embedding, OkhsvEmbedding},
    pre_embed,
    renders::{okhsv_embed_oklab, ColorChip, Cursor, Renderable},
    InputState, Renderer,
};

pub struct Target<'a> {
    pub target: &'a RenderTarget<'a>,
    pub program: &'a mut Program,
    pub pos_target: &'a RenderTarget<'a>,
    pub pos_program: &'a mut Program,
}

pub trait Scene<T> {
    fn update(&mut self, state: T);
    fn render(&self, target: &mut Target, state: T);
}

pub struct ColorScene {
    cursor: Cursor,
    chip: ColorChip,
    elements: Vec<Box<dyn ColorElement<InputState>>>,
}

impl ColorScene {
    fn new<T: Embedding + 'static, U: Embedding + 'static>(
        context: &Context,
        space_embedding: T,
        color_embedding: U,
    ) -> Self {
        let space = pre_embed::cylinder(48, 6, 2);
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
            ],
            chip: ColorChip::new(&context),
        }
    }

    pub fn cylinder(context: &Context) -> Self {
        Self::new(context, CylindricalEmbedding {}, OkhsvEmbedding {})
    }

    pub fn render_graph(&self, target: &mut Target, graph: &ModelGraph) {
        match graph {
            ModelGraph::Color(model) => {
                target.program.render(target.target, model);
            }
            ModelGraph::Space(model) => {
                let tagged_model = TaggedColorModel {
                    model: model,
                    tag: 1,
                };
                target.pos_program.render(target.pos_target, &tagged_model);
            }
            ModelGraph::Vec(models) => {
                for model in models {
                    self.render_graph(target, model);
                }
            }
        }
    }
}

impl Scene<&InputState> for ColorScene {
    fn update(&mut self, state: &InputState) {
        for element in &mut self.elements {
            element.update(state);
        }
    }

    fn render(&self, target: &mut Target, state: &InputState) {
        let screen = target.target;
        for element in &self.elements {
            let model_graph = element.model();
            self.render_graph(target, &model_graph);
        }
        target.program.render(screen, &self.cursor.model(state));
        target
            .program
            .render(screen, &self.chip.model(&okhsv_embed_oklab(state.pos)));
    }
}
