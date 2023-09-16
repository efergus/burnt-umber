extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use three_d::{
    renderer::{control::Event, render_states::*, *},
    FrameInput, FrameInputGenerator, FrameOutput, SurfaceSettings, Window, WindowedContext,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use winit::event_loop::EventLoop;
// use

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn polar_cylinder(subdivisions: i32) -> (Vec<Vector3<f32>>, Vec<Srgba>) {
    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut colors: Vec<Srgba> = Vec::new();
    let top = Vec3::new(1.0, 0.0, 0.0);
    let bottom = Vec3::new(0.0, 0.0, 0.0);
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left_bottom = Vec3::new(0.0, angle.cos(), angle.sin());
        let left_top = Vec3::new(1.0, angle.cos(), angle.sin());
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right_bottom = Vec3::new(0.0, angle.cos(), angle.sin());
        let right_top = Vec3::new(1.0, angle.cos(), angle.sin());
        let left_turn = (left_turn * 255.0) as u8;
        let right_turn = (right_turn * 255.0) as u8;
        positions.push(top);
        colors.push(Srgba::new_opaque(left_turn, 255, 0));
        positions.push(left_top);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(right_top);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));

        positions.push(left_top);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(left_bottom);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(right_bottom);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));

        positions.push(left_top);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(right_bottom);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(right_top);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));

        positions.push(bottom);
        colors.push(Srgba::new_opaque(right_turn, 0, 0));
        positions.push(right_bottom);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(left_bottom);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));
    }
    (positions, colors)
}

fn polar_cone(subdivisions: i32) -> (Vec<Vector3<f32>>, Vec<Srgba>) {
    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut colors: Vec<Srgba> = Vec::new();
    let top = Vec3::new(1.0, 0.0, 0.0);
    let bottom = Vec3::new(0.0, 0.0, 0.0);
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left = Vec3::new(0.0, angle.cos(), angle.sin());
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right = Vec3::new(0.0, angle.cos(), angle.sin());
        let left_turn = (left_turn * 255.0) as u8;
        let right_turn = (right_turn * 255.0) as u8;
        positions.push(top);
        colors.push(Srgba::new_opaque(left_turn, 255, 0));
        positions.push(left);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(right);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));
        positions.push(left);
        colors.push(Srgba::new_opaque(left_turn, 0, 255));
        positions.push(bottom);
        colors.push(Srgba::new_opaque(right_turn, 0, 0));
        positions.push(right);
        colors.push(Srgba::new_opaque(right_turn, 0, 255));
    }
    (positions, colors)
}

fn quad_mesh() -> Vec<Vec3> {
    return vec![
        vec3(0.0, 0.0, 0.0),
        vec3(1.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 1.0, 0.0),
    ];
}

// fn sphere(subdivisions: u32) -> Vec<Vec3> {

// }
// struct Uniform<T: UniformDataType> {
//     name: String,
//     value: T,
// }

struct Model {
    positions: VertexBuffer,
    transform: Mat4,
}

// trait Renderable {
//     fn positions() -> Vec<Vec3>;
// }

trait Renders {
    fn default_program(&self) -> &Program;
    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }
    fn render(&self, target: &RenderTarget, model: &Model, view: Mat4) {
        self.render_with_meta(target, model, view, model.transform)
    }
    fn render_with_meta(&self, target: &RenderTarget, model: &Model, view: Mat4, meta: Mat4) {
        target.write(move || {
            let program = &self.default_program();
            program.use_uniform("model", model.transform);
            program.use_uniform_if_required("meta", meta);
            program.use_uniform("view", view);
            program.use_vertex_attribute("position", &model.positions);
            program.draw_arrays(
                self.render_states(),
                target.viewport(),
                model.positions.vertex_count(),
            );
        });
    }
}

struct Scene {
    program: Program,
}

impl Scene {
    fn new(context: &Context, fragment_shader_source: &str) -> Self {
        let program = Program::from_source(
            context,
            include_str!("color.vert"),
            fragment_shader_source, // include_str!("hsv.frag"),
        )
        .unwrap();
        Scene { program }
    }
}

impl Renders for Scene {
    fn default_program(&self) -> &Program {
        &self.program
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            cull: Cull::Back,
            depth_test: DepthTest::Always,
            ..Default::default()
        }
    }
}

#[wasm_bindgen]
pub struct ColorView {
    // winit_window: winit::window::Window,
    // context: WindowedContext,
    // event_loop: EventLoop<()>,
    window: Window,
    width: u32,
    height: u32,
    control: OrbitControl,
    selection: Vec3,
    camera: Camera,
    color_scene: Scene,
    pos_scene: Scene,
    cube: Model,
    cylinder: Model,
    quad: Model,
    on_select: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
    on_hover: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
}

#[wasm_bindgen]
impl ColorView {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        match canvas.dyn_into::<HtmlCanvasElement>() {
            Ok(canvas) => {
                #[cfg(not(target_arch = "wasm32"))]
                let window_builder = winit::window::WindowBuilder::new()
                    .with_title("winit window")
                    .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
                    .with_maximized(true);
                #[cfg(target_arch = "wasm32")]
                let window_builder = {
                    use winit::platform::web::WindowBuilderExtWebSys;
                    winit::window::WindowBuilder::new()
                        .with_canvas(Some(canvas))
                        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                };

                let event_loop = winit::event_loop::EventLoop::new();
                let winit_window = window_builder.build(&event_loop).unwrap();
                let window = Window::from_winit_window(
                    winit_window,
                    event_loop,
                    SurfaceSettings::default(),
                    false,
                )
                .unwrap();
                let context = window.gl();
                // let context = WindowedContext::from_winit_window(&winit_window, SurfaceSettings::default()).unwrap();

                let camera = Camera::new_perspective(
                    Viewport::new_at_origo(1, 1),
                    vec3(0.0, 2.0, 4.0),
                    vec3(0.0, 0.5, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    degrees(45.0),
                    0.1,
                    10.0,
                );
                let control = OrbitControl::new(*camera.target(), 1.0, 100.0);

                let color_scene = Scene::new(&context, include_str!("hsv.frag"));
                let pos_scene = Scene::new(&context, include_str!("pos.frag"));
                let (cube, cylinder, quad) = ColorView::initialize_models(&context);
                let view = ColorView {
                    window,
                    // winit_window,
                    // context,
                    // event_loop,
                    width,
                    height,
                    control,
                    selection: vec3(0.0, 0.0, 0.0),
                    on_select: None,
                    on_hover: None,
                    camera,
                    color_scene,
                    pos_scene,
                    cube,
                    cylinder,
                    quad,
                };
                view
            }
            _ => panic!("ColorView::new must be passed a canvas!"),
        }
    }

    fn initialize_models(context: &Context) -> (Model, Model, Model) {
        let cube = VertexBuffer::new_with_data(&context, &CpuMesh::cube().positions.to_f32());
        let (positions, colors) = polar_cylinder(64);
        let cylinder = VertexBuffer::new_with_data(&context, &positions);
        let quad = VertexBuffer::new_with_data(&context, &quad_mesh());
        (
            Model {
                positions: cube,
                transform: Mat4::from_scale(0.5) * Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0)),
            },
            Model {
                positions: cylinder,
                transform: Mat4::from_translation(Vector3::new(0.0, 1.0, 0.0))
                    * Mat4::from_angle_z(degrees(-90.0)),
            },
            Model {
                positions: quad,
                transform: Mat4::identity(),
            },
        )
    }

    pub fn render_loop(mut self) {
        // let mut input_generator = FrameInputGenerator::from_winit_window(&self.winit_window);
        let context = self.window.gl();
        // self.render(input);
        // let view = Rc::new(RefCell::new(self));
        // let event_loop = Rc::new(RefCell::new(self.event_loop));

        self.window.render_loop(move |mut input| {
            let mut position: Option<Vector2<f32>> = None;
            for event in input.events.iter() {
                match event {
                    Event::MouseMotion { position: pos, .. } => {
                        position = Some(Vec2::new(pos.x, pos.y));
                        self.selection.x = pos.x;
                        self.selection.y = pos.y;
                    }
                    _ => {}
                }
            }
            self.control
                .handle_events(&mut self.camera, &mut input.events);
            let screen = input.screen();
            let view = self.camera.projection() * self.camera.view();
            self.color_scene.render(&screen, &self.cylinder, view);
            let quad_meta = Mat4::from_angle_y(radians(input.accumulated_time as f32 * 0.001))
                * Mat4::from_translation(vec3(1.0, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(0.0, 1.0, 1.0);
            let quad_view = Mat4::from_translation(vec3(-1.0, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(0.2, 1.0, 1.0);
            self.color_scene
                .render_with_meta(&screen, &self.quad, quad_view, quad_meta);
            if let Some(position) = position {
                let mut texture = Texture2D::new_empty::<[f32; 4]>(
                    &context,
                    self.width,
                    self.height,
                    Interpolation::Nearest,
                    Interpolation::Nearest,
                    None,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                let mut depth_texture = DepthTexture2D::new::<f32>(
                    &context,
                    self.width,
                    self.height,
                    Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge,
                );
                let pos_target = RenderTarget::new(
                    texture.as_color_target(None),
                    depth_texture.as_depth_target(),
                );
                pos_target.clear(ClearState::depth(1.0));
                self.pos_scene.render(&pos_target, &self.cylinder, view);
                self.pos_scene
                    .render_with_meta(&pos_target, &self.quad, quad_view, quad_meta);
                // self.pos_scene.render(&screen, &self.cylinder, view);
                // self.pos_scene.render(&screen, &self.quad, quad_view);
                let scissor_box = ScissorBox {
                    x: position.x as i32,
                    y: (self.height as i32) - (position.y as i32),
                    width: 1,
                    height: 1,
                };
                let pos = pos_target.read_color_partially::<[f32; 4]>(scissor_box)[0];
                self.selection = vec3(pos[0], pos[1], pos[2]);
                // log(&format!("{}, {}, {:?} {:?}", self.width, self.height, position, pos));
            }
            self.cube.transform = Mat4::from_translation(self.selection) * Mat4::from_scale(0.05);
            self.color_scene.render(&screen, &self.cube, view);
            FrameOutput::default()
        });
    }
}
