extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;

use renders::{AxisInput, ColorSpace, Cursor};
use winit::window::WindowBuilder;
mod geometry;
mod renders;

use three_d::{
    renderer::{control::Event, *},
    FrameOutput, SurfaceSettings, Window,
};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use crate::renders::{InputState, Renderable, Renderer, Space};
// use

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn to_cylindrical(v: Vec3) -> Vec3 {
    let angle = v.z.atan2(v.x);
    let radius = vec2(v.x, v.z).magnitude();
    let y = v.y;
    vec3(angle, radius, y)
}

fn from_cylindrical(v: Vec3) -> Vec3 {
    let x = v.x.cos() * v.y;
    let z = v.x.sin() * v.y;
    let y = v.z;
    vec3(x, y, z)
}

struct CustomController {
    control: CameraControl,
    distance: Vec2,
}
impl CustomController {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: Self::create_controller(target, min_distance, max_distance),
            distance: vec2(min_distance, max_distance),
        }
    }

    fn create_controller(target: Vec3, min_distance: f32, max_distance: f32) -> CameraControl {
        CameraControl {
            left_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.1 },
            left_drag_vertical: CameraAction::OrbitUp { target, speed: 0.1 },
            scroll_vertical: CameraAction::Zoom {
                min: min_distance,
                max: max_distance,
                speed: 0.01,
                target,
            },
            ..Default::default()
        }
    }

    pub fn set_target(&mut self, camera: &mut Camera, target: Vec3) {
        let position = *camera.position();
        let old_target = *camera.target();
        camera.set_view(target + position - old_target, target, vec3(0.0, 1.0, 0.0));
        self.control = Self::create_controller(target, self.distance.x, self.distance.y)
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        if let CameraAction::Zoom { speed, target, .. } = &mut self.control.scroll_vertical {
            let x = target.distance(*camera.position());
            *speed = 0.001 * x + 0.001;
        }
        if let CameraAction::OrbitLeft { speed, target } = &mut self.control.left_drag_horizontal {
            let x = target.distance(*camera.position());
            *speed = 0.01 * x + 0.001;
        }
        if let CameraAction::OrbitUp { speed, target } = &mut self.control.left_drag_vertical {
            let x = target.distance(*camera.position());
            *speed = 0.01 * x + 0.001;
        }
        self.control.handle_events(camera, events)
    }
}

struct Target<'a> {
    target: &'a RenderTarget<'a>,
    program: &'a mut Program,
    pos_target: &'a RenderTarget<'a>,
    pos_program: &'a mut Program,
}

trait Scene<T> {
    fn render(&self, target: &mut Target, state: T);
}

struct ColorScene {
    cursor: Cursor,
    space: ColorSpace,
    axes: [AxisInput; 3],
}

impl ColorScene {
    fn new(context: &Context, color_space: ColorSpace) -> Self {
        Self {
            cursor: Cursor::cube(&context),
            space: color_space,
            axes: [
                AxisInput::new(&context, 0),
                AxisInput::new(&context, 1),
                AxisInput::new(&context, 2),
            ],
        }
    }

    fn cylinder(context: &Context) -> Self {
        Self::new(context, ColorSpace::cylinder(context))
    }

    fn cube(context: &Context) -> Self {
        Self::new(context, ColorSpace::cube(context))
    }
}

impl Scene<&InputState> for ColorScene {
    fn render(&self, target: &mut Target, state: &InputState) {
        let space = &self.space.model(state);
        let screen = target.target;
        target.program.render(screen, space);
        target.program.render(screen, &self.cursor.model(state));
        for i in 0..3 {
            target.program.render(screen, &self.axes[i].model(state));
        }
        let screen = target.pos_target;
        target.pos_program.render(screen, space);
        for i in 0..3 {
            target
                .pos_program
                .render(screen, &self.axes[i].model(state));
        }
    }
}

#[wasm_bindgen]
pub struct ColorView {
    window: Window,
    // width: u32,
    height: u32,
    control: CustomController,
    position: Vec2,
    state: InputState,
    cylindrical_program: Program,
    linear_program: Program,
    pos_program: Program,
    cylindrical_scene: ColorScene,
    linear_scene: ColorScene,
    pos_texture: Texture2D,
    depth_texture: DepthTexture2D,
    on_select: Option<Box<dyn FnMut(Vec3) -> ()>>,
    // on_hover: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
}

fn color_shader(string: &str) -> String {
    include_str!("color.frag").replace("// REPLACE", string)
}

fn color_program(context: &Context, string: &str) -> Program {
    let src = color_shader(string);
    Program::from_source(&context, include_str!("color.vert"), &src).unwrap()
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
        let control = CustomController::new(*camera.target(), 1.0, 100.0);

        let cylindrical_program =
            color_program(&context, "color = vec4(hsv2rgb(xyz2hsv(pos.xyz)), 1.0);");
        let linear_program = color_program(&context, "color = vec4(pos.xyz, 1.0);");
        let pos_program = color_program(&context, "color = vec4(pos.xyz, tag);");
        let pos_texture = Texture2D::new_empty::<[f32; 4]>(
            &context,
            width,
            height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let depth_texture = DepthTexture2D::new::<f32>(
            &context,
            width,
            height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let view = ColorView {
            window,
            // width,
            height,
            control,
            position: vec2(0.0, 0.0),
            on_select: None,
            // on_hover: None,
            state: InputState::new(vec3(1.0, 1.0, 1.0), camera, Space::Linear),
            cylindrical_program,
            linear_program,
            pos_program,
            cylindrical_scene: ColorScene::cylinder(&context),
            linear_scene: ColorScene::cube(&context),
            pos_texture,
            depth_texture,
        };
        view
    }

    pub fn render_loop(mut self) {
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
                .handle_events(&mut self.state.camera, &mut input.events);
            let screen = input.screen();
            let state = &self.state;
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.0, 1.0));
            let pos_target = RenderTarget::new(
                self.pos_texture.as_color_target(None),
                self.depth_texture.as_depth_target(),
            );
            pos_target.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0));
            let (program, scene) = if self.state.space == Space::Linear {
                (&mut self.linear_program, &self.linear_scene)
            } else {
                (&mut self.cylindrical_program, &self.cylindrical_scene)
            };
            let mut target = Target {
                target: &screen,
                program,
                pos_target: &pos_target,
                pos_program: &mut self.pos_program,
            };
            scene.render(&mut target, state);
            let position = self.position;
            let scissor_box = ScissorBox {
                x: position.x as i32,
                y: (self.height as i32) - (position.y as i32),
                width: 1,
                height: 1,
            };
            let pos = pos_target.read_color_partially::<[f32; 4]>(scissor_box)[0];
            let tag = pos[3] as u8;
            let mut pos = vec3(pos[0], pos[1], pos[2]);
            let mut pos_state = self.state.pos;
            let mut saved = self.state.saved_pos;
            if self.state.space == Space::Cylindrical {
                pos = to_cylindrical(pos);
                pos_state = self.state.cylindrical;
                saved = self.state.saved_pos;
            }
            let pos = match tag {
                1 => vec3(pos.x, pos_state.y, pos_state.z),
                2 => vec3(pos_state.x, pos.y, pos_state.z),
                3 => vec3(pos_state.x, pos_state.y, pos.z),
                7 => pos,
                _ => saved,
            };
            self.state.chunk = match tag {
                1 => vec3(pos.x, self.state.chunk.y, self.state.chunk.z),
                2 => vec3(self.state.chunk.x, pos.y, self.state.chunk.z),
                3 => vec3(self.state.chunk.x, self.state.chunk.y, pos.z),
                _ => self.state.chunk,
            };
            // log(&format!("{:?} {:?}", self.state.chunk, pos));
            if self.state.space == Space::Cylindrical {
                self.state.cylindrical = pos;
                self.state.pos = from_cylindrical(pos);
            } else {
                self.state.cylindrical = to_cylindrical(pos);
                self.state.pos = pos;
            }
            if press {
                self.state.saved_cylindrical = self.state.cylindrical;
                self.state.saved_pos = self.state.pos;
                if let Some(on_select) = self.on_select.as_mut() {
                    on_select(self.state.pos);
                }
            }
            FrameOutput::default()
        });
    }

    pub fn set_space(&mut self, space: String) {
        self.state.space = match space.as_str() {
            "linear" => Space::Linear,
            "cylindrical" => Space::Cylindrical,
            _ => panic!("Expected 'linear' or 'cylindrical"),
        };
        let new_target = match self.state.space {
            Space::Linear => vec3(0.5, 0.5, 0.5),
            Space::Cylindrical => vec3(0.0, 0.5, 0.0),
        };
        self.control.set_target(&mut self.state.camera, new_target)
    }
}
