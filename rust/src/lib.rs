extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;

use std::f32::consts::PI;

use camera::CustomController;
use scene::ColorScene;
use winit::window::WindowBuilder;
mod camera;
mod geometry;
mod mesh;
mod renders;
mod scene;

use three_d::{
    renderer::{control::Event, *},
    FrameOutput, SurfaceSettings, Window,
};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

pub use crate::renders::{InputState, Renderable, Renderer, Space};
use crate::scene::{Scene, Target};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn from_cylindrical(v: Vec3) -> Vec3 {
    let h = v.x * PI * 2.0;
    let x = h.cos() * v.z;
    let z = h.sin() * v.z;
    let y = v.y;
    vec3(x, y, z)
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
    pos_program: Program,
    cylindrical_scene: ColorScene,
    pos_texture: Texture2D,
    depth_texture: DepthTexture2D,
    on_select: Option<Box<dyn FnMut(Vec3)>>,
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
        ColorView::build(window_builder, width, height, None)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        canvas: HtmlCanvasElement,
        width: u32,
        height: u32,
        callback: js_sys::Function,
    ) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let window_builder = match canvas.dyn_into::<HtmlCanvasElement>() {
            Ok(canvas) => {
                use winit::platform::web::WindowBuilderExtWebSys;
                winit::window::WindowBuilder::new()
                    .with_canvas(Some(canvas))
                    .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            }
            _ => panic!("ColorView::new must be passed a canvas!"),
        };
        let f: Box<dyn FnMut(Vec3)> = Box::new(move |v: Vec3| {
            let this = JsValue::null();
            let _ = callback.call3(
                &this,
                &JsValue::from(v.x),
                &JsValue::from(v.y),
                &JsValue::from(v.z),
            );
        });
        let on_select = Some(f);
        ColorView::build(window_builder, width, height, on_select)
    }

    fn build(
        window_builder: WindowBuilder,
        width: u32,
        height: u32,
        on_select: Option<Box<dyn FnMut(Vec3)>>,
    ) -> ColorView {
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

        // let cylindrical_program =
        //     color_program(&context, "color = vec4(hsv2rgb(xyz2hsv(pos.xyz)), 1.0);");
        let cylindrical_program =
            color_program(&context, "color = vec4(oklab_to_srgb(pos.xyz), 1.0);");
        // let cylindrical_program =
        //     color_program(&context, "color = vec4(linear_srgb_to_oklab(pos.yxz), 1.0);");
        // let cylindrical_program = color_program(&context, "color = vec4(linear_srgb_to_srgb(pos.xyz), 1.0);");

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
        let state = InputState::new(vec3(1.0, 1.0, 1.0), camera);
        let mut cylindrical_scene = ColorScene::cylinder(&context);
        cylindrical_scene.update(&state);
        let view = ColorView {
            window,
            // width,
            height,
            control,
            position: vec2(0.0, 0.0),
            on_select,
            // on_hover: None,
            state,
            cylindrical_program,
            pos_program,
            cylindrical_scene,
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
            let state = &mut self.state;
            state.input = true;
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.0, 1.0));
            let pos_target = RenderTarget::new(
                self.pos_texture.as_color_target(None),
                self.depth_texture.as_depth_target(),
            );
            pos_target.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0));

            let program = &mut self.cylindrical_program;
            let scene = &mut self.cylindrical_scene;
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
            let pos = vec3(pos[0], pos[1], pos[2]);
            let pos_state = state.pos;

            state.pos = match tag {
                1 => vec3(pos.x, pos_state.y, pos_state.z),
                2 => vec3(pos_state.x, pos.y, pos_state.z),
                3 => vec3(pos_state.x, pos_state.y, pos.z),
                7 => pos,
                _ => state.saved_pos,
            };
            state.chunk = match tag {
                1 => vec3(pos.x, state.chunk.y, state.chunk.z),
                2 => vec3(state.chunk.x, pos.y, state.chunk.z),
                3 => vec3(state.chunk.x, state.chunk.y, pos.z),
                _ => state.chunk,
            };

            if press && tag > 0 {
                state.saved_pos = pos;
            }
            if state.pos != pos_state {
                if let Some(on_select) = self.on_select.as_mut() {
                    on_select(state.pos);
                }
                scene.update(state);
            }
            FrameOutput::default()
        });
    }
}
