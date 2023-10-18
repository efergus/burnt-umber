use std::rc::Rc;

use three_d::{Context, Program, RenderTarget};

use crate::{
    element::{
        coloraxis::{Axis, ColorAxis},
        colorspace::ColorSpace,
        ColorElement,
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
    space: ColorSpace,
    axes: [ColorAxis; 3],
    chip: ColorChip,
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
            space: space,
            axes: [
                ColorAxis::new(&context, Axis::X, color_embedding.clone()),
                ColorAxis::new(&context, Axis::Y, color_embedding.clone()),
                ColorAxis::new(&context, Axis::Z, color_embedding.clone()),
            ],
            chip: ColorChip::new(&context),
        }
    }

    pub fn cylinder(context: &Context) -> Self {
        Self::new(context, CylindricalEmbedding {}, OkhsvEmbedding {})
    }
}

impl Scene<&InputState> for ColorScene {
    fn update(&mut self, state: &InputState) {
        self.space.update(state);
        for i in 0..3 {
            self.axes[i].update(state);
        }
    }

    fn render(&self, target: &mut Target, state: &InputState) {
        let screen = target.target;
        target.program.render(screen, &self.space.color_model());
        target.program.render(screen, &self.cursor.model(state));
        for i in 0..3 {
            target.program.render(screen, &self.axes[i].color_model());
        }
        target
            .program
            .render(screen, &self.chip.model(&okhsv_embed_oklab(state.pos)));

        let screen = target.pos_target;
        target.pos_program.render(screen, &self.space.space_model());
        for i in 0..3 {
            target
                .pos_program
                .render(screen, &self.axes[i].space_model());
        }
    }
}
