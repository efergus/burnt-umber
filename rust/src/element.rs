use three_d::{ElementBuffer, Mat4, RenderStates, Vec3, VertexBuffer};

pub mod coloraxis;
pub mod colorspace;

pub struct ColorModel<'a> {
    pub positions: &'a VertexBuffer,
    pub embed: &'a VertexBuffer,
    pub indices: &'a ElementBuffer,
    pub render_states: RenderStates,
    pub view: Mat4,
    pub model: Mat4,
    pub meta: Mat4,
}

pub struct SpaceModel<'a> {
    pub positions: &'a VertexBuffer,
    pub embed: &'a VertexBuffer,
    pub indices: &'a ElementBuffer,
    pub render_states: RenderStates,
    pub view: Mat4,
    pub model: Mat4,
    pub meta: Mat4,
}

pub trait ColorElement<T> {
    fn update(&mut self, state: &T);
    fn color_model(&self) -> ColorModel;
    fn space_model(&self) -> SpaceModel;
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
