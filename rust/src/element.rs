use three_d::{ElementBuffer, Mat4, RenderStates, Vec3, VertexBuffer};

pub mod coloraxis;
pub mod colorspace;

pub enum ModelGraph<'a> {
    Color(ColorModel<'a>),
    Space(ColorModel<'a>),
    Vec(Vec<ModelGraph<'a>>),
}

pub struct ColorModel<'a> {
    pub positions: &'a VertexBuffer,
    pub embed: &'a VertexBuffer,
    pub indices: &'a ElementBuffer,
    pub render_states: RenderStates,
    pub view: Mat4,
    pub model: Mat4,
    pub meta: Mat4,
}

pub struct TaggedColorModel<'a> {
    pub model: &'a ColorModel<'a>,
    pub tag: u16,
}

pub trait ColorElement<T> {
    fn update(&mut self, state: &T);
    fn model(&self) -> ModelGraph;
    fn invert_space(&self, pos: Vec3) -> Vec3;
}

// impl<'a, T, U: ColorElement<T>> Renderable<T, ColorModel<'a>> for U {
//     fn model(&self, state: &T) -> ColorModel<'a> {
//         self.color_model()
//     }
// }
// impl<'a, T, U: ColorElement<T>> Renderable<T, SpaceModel<'a>> for U {
//     fn model(&self, state: &T) -> SpaceModel<'a> {
//         self.space_model()
//     }
// }
