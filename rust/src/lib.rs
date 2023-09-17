extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use winit::window::WindowBuilder;
mod geometry;
use geometry::{cylinder_mesh, quad_mesh};

use three_d::{
    renderer::{control::Event, render_states::*, *},
    FrameOutput, SurfaceSettings, Window,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
// use

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
        self.render_with_meta(target, model, view, model.transform, 5.0)
    }
    fn render_with_meta(
        &self,
        target: &RenderTarget,
        model: &Model,
        view: Mat4,
        meta: Mat4,
        tag: f32,
    ) {
        target.write(move || {
            let program = &self.default_program();
            program.use_uniform("model", model.transform);
            program.use_uniform_if_required("meta", meta);
            program.use_uniform("view", view);
            program.use_uniform_if_required("tag", tag);
            program.use_vertex_attribute("position", &model.positions);
            program.use_vertex_attribute("embed", &model.positions);
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
        let program =
            Program::from_source(context, include_str!("color.vert"), fragment_shader_source)
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
    hover: Vec3,
    chunk: Vec3,
    position: Vec2,
    // tags: Vec<Box<dyn FnMut(&mut ColorView, Vec3)->()>,
    camera: Camera,
    color_scene: Scene,
    pos_scene: Scene,
    cube: Model,
    cylinder: Model,
    quad: Model,
    on_select: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
    // on_hover: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
}

fn color_shader(string: &str) -> String {
    include_str!("color.frag").replace("// REPLACE", string)
}

fn set_with_tag(source: Vec3, new: Vec3, tag: u8) -> Vec3 {
    let mut dest = source;
    if (tag & 1) != 0 {
        dest.x = new.x;
    }
    if (tag & 2) != 0 {
        dest.y = new.y;
    }
    if (tag & 4) != 0 {
        dest.z = new.z;
    }
    dest
}

#[wasm_bindgen]
impl ColorView {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(width: u32, height: u32) -> Self {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("winit window")
            .with_min_inner_size(winit::dpi::LogicalSize::new(width, height))
            .with_maximized(true);
        ColorView::build(window_builder, width, height)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        let window_builder = match canvas.dyn_into::<HtmlCanvasElement>() {
            Ok(canvas) => {
                use winit::platform::web::WindowBuilderExtWebSys;
                winit::window::WindowBuilder::new()
                    .with_canvas(Some(canvas))
                    .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            }
            _ => panic!("ColorView::new must be passed a canvas!"),
        };
        ColorView::build(window_builder, width, height)
    }

    fn build(window_builder: WindowBuilder, width: u32, height: u32) -> ColorView {
        let event_loop = winit::event_loop::EventLoop::new();
        let winit_window = window_builder.build(&event_loop).unwrap();
        let window =
            Window::from_winit_window(winit_window, event_loop, SurfaceSettings::default(), false)
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

        let hsv_shader = color_shader("color = vec4(hsv2rgb(xyz2hsv(pos.xyz)), 1.0);");
        let color_scene = Scene::new(&context, &hsv_shader);
        let pos_shader = color_shader("color = vec4(pos.xyz, tag);");
        let pos_scene = Scene::new(&context, &pos_shader);
        let (cube, cylinder, quad) = ColorView::initialize_models(&context);
        // let tags = vec![
        //     Box::new(|view: &mut Self, color: Vec3| {

        //     })
        // ];
        let view = ColorView {
            window,
            // winit_window,
            // context,
            // event_loop,
            width,
            height,
            control,
            selection: vec3(0.0, 0.0, 1.0),
            hover: vec3(0.0, 0.0, 1.0),
            chunk: vec3(1.0, 1.0, 1.0),
            position: vec2(0.0, 0.0),
            // tags,
            on_select: None,
            // on_hover: None,
            camera,
            color_scene,
            pos_scene,
            cube,
            cylinder,
            quad,
        };
        view
    }

    fn initialize_models(context: &Context) -> (Model, Model, Model) {
        let cube = VertexBuffer::new_with_data(&context, &CpuMesh::cube().positions.to_f32());
        let cylinder = VertexBuffer::new_with_data(&context, &cylinder_mesh(64));
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
            let mut press = false;
            for event in input.events.iter() {
                match event {
                    Event::MouseMotion { position: pos, .. } => {
                        self.position = Vec2::new(pos.x, pos.y);
                    }
                    Event::MousePress {
                        button,
                        position: pos,
                        ..
                    } => {
                        self.position = Vec2::new(pos.x, pos.y);
                        press = *button == MouseButton::Left;
                    }
                    _ => {}
                }
            }
            self.control
                .handle_events(&mut self.camera, &mut input.events);
            let screen = input.screen();
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.0, 1.0));
            let view = self.camera.projection() * self.camera.view();
            self.cylinder.transform = Mat4::from_nonuniform_scale(1.0, self.chunk.y, 1.0)
                * Mat4::from_translation(Vector3::new(0.0, 1.0, 0.0))
                * Mat4::from_angle_z(degrees(-90.0));
            self.color_scene.render(&screen, &self.cylinder, view);
            // let quad_meta = Mat4::from_angle_y(radians(input.accumulated_time as f32 * 0.001))
            let quad_meta = Mat4::from_translation(vec3(self.hover.x, 0.0, self.hover.z))
                * Mat4::from_nonuniform_scale(0.0, 1.0, 1.0);
            let quad_view = Mat4::from_translation(vec3(-1.0, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(0.2, 1.0, 1.0);
            self.color_scene
                .render_with_meta(&screen, &self.quad, quad_view, quad_meta, 7.0);
            let sample_meta = Mat4::from_translation(self.hover) * Mat4::from_scale(0.0);
            let sample_view = Mat4::from_translation(vec3(0.5, 0.5, 0.0));
            self.color_scene
                .render_with_meta(&screen, &self.quad, sample_view, sample_meta, 0.0);
            let position = self.position;
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
                .render_with_meta(&pos_target, &self.quad, quad_view, quad_meta, 7.0);
            // self.pos_scene.render(&screen, &self.cylinder, view);
            // self.pos_scene.render(&screen, &self.quad, quad_view);
            let scissor_box = ScissorBox {
                x: position.x as i32,
                y: (self.height as i32) - (position.y as i32),
                width: 1,
                height: 1,
            };
            let pos = pos_target.read_color_partially::<[f32; 4]>(scissor_box)[0];
            let select = pos[3] as u8;
            let pos = vec3(pos[0], pos[1], pos[2]);
            if select != 0 {
                if press {
                    self.selection = pos;
                }
                self.hover = pos;
            } else {
                self.hover = self.selection;
            }
            if let Some(on_select) = self.on_select.as_mut() {
                on_select(self.hover.x, self.hover.y, self.hover.z);
            }
            self.chunk = set_with_tag(self.chunk, pos, select);
            self.cube.transform = Mat4::from_translation(self.hover) * Mat4::from_scale(0.05);
            self.color_scene.render(&screen, &self.cube, view);
            FrameOutput::default()
        });
    }
}
