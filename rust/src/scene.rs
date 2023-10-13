use three_d::{Context, Program, RenderTarget};

use crate::{
    renders::{okhsv_embed_oklab, Axis, AxisInput, ColorChip, ColorSpace, Cursor},
    InputState, Renderable, Renderer,
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
    axes: [AxisInput; 3],
    chip: ColorChip,
}

impl ColorScene {
    fn new(context: &Context, color_space: ColorSpace) -> Self {
        Self {
            cursor: Cursor::cube(&context),
            space: color_space,
            axes: [
                AxisInput::new(&context, Axis::X),
                AxisInput::new(&context, Axis::Y),
                AxisInput::new(&context, Axis::Z),
            ],
            chip: ColorChip::new(&context),
        }
    }

    pub fn cylinder(context: &Context) -> Self {
        Self::new(context, ColorSpace::cylinder(context))
    }
}

impl Scene<&InputState> for ColorScene {
    fn update(&mut self, state: &InputState) {
        self.space.okhsv_embed_quads(state.chunk);
        for i in 0..3 {
            self.axes[i].update(state.pos, okhsv_embed_oklab);
        }
    }

    fn render(&self, target: &mut Target, state: &InputState) {
        let screen = target.target;
        target.program.render(screen, &self.space.model(state));
        target.program.render(screen, &self.cursor.model(state));
        for i in 0..3 {
            target.program.render(screen, &self.axes[i].model(state));
        }
        target
            .program
            .render(screen, &self.chip.model(&okhsv_embed_oklab(state.pos)));

        let screen = target.pos_target;
        target.pos_program.render(screen, &self.space.model(state));
        for i in 0..3 {
            target
                .pos_program
                .render(screen, &self.axes[i].model(state));
        }
    }
}
