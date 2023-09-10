extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use three_d::{renderer::{*, render_states::*, control::Event}, SurfaceSettings, Window, FrameInput, FrameOutput};
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
    let top = Vec3::new(
        1.0,
        0.0,
        0.0,
    );
    let bottom = Vec3::new(
        0.0,
        0.0,
        0.0,
    );
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left_bottom = Vec3::new(
            0.0,
            angle.cos(),
            angle.sin(),
        );
        let left_top = Vec3::new(
            1.0,
            angle.cos(),
            angle.sin(),
        );
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right_bottom = Vec3::new(
            0.0,
            angle.cos(),
            angle.sin(),
        );
        let right_top = Vec3::new(
            1.0,
            angle.cos(),
            angle.sin(),
        );
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
    let top = Vec3::new(
        1.0,
        0.0,
        0.0,
    );
    let bottom = Vec3::new(
        0.0,
        0.0,
        0.0,
    );
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left = Vec3::new(
            0.0,
            angle.cos(),
            angle.sin(),
        );
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right = Vec3::new(
            0.0,
            angle.cos(),
            angle.sin(),
        );
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

// fn sphere(subdivisions: u32) -> Vec<Vec3<f32>> {

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
    fn default_program(&self) -> Program;
    fn render(&self, target: &RenderTarget, model: Model, view: Mat4) -> &RenderTarget {
        target
            .write(|| {
                let program = self.default_program();
                program.use_uniform(
                    "model",
                    model.transform
                );
                program.use_uniform(
                    "view",
                    view
                );
                program.use_vertex_attribute("position", &model.positions);
                program.draw_arrays(RenderStates::default(), target.viewport(), model.positions.vertex_count());
            })
    };
}

struct Scene {
    program: Program,
}

impl Scene {
    fn new(context: &Context, fragment_shader_source: &str) -> Self {
        let program = Program::from_source(
            context, 
            include_str!("color.vert"),
            fragment_shader_source
            // include_str!("hsv.frag"),
        ).unwrap();
        Scene { program }
    }
}

impl Renders for Scene {
    fn default_program(&self) -> Program {
        self.program
    }
}

#[wasm_bindgen]
pub struct ColorView {
    window: Window,
    selection: Vec3,
    camera: Camera,
    color_scene: Scene,
    pos_scene: Scene,
    cube: Model,
    cylinder: Model,
    on_select: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
    on_hover: Option<Box<dyn FnMut(f32, f32, f32) -> ()>>,
}

#[wasm_bindgen]
impl ColorView {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
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
                        .with_canvas(Some(
                            canvas
                        ))
                        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                        // .with_prevent_default(true)
                };

                let event_loop = winit::event_loop::EventLoop::new();
                let winit_window = window_builder.build(&event_loop).unwrap();
                // let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();
                let window = Window::from_winit_window(winit_window, event_loop, SurfaceSettings::default(), false).unwrap();
                let context = window.gl();
                let (program, position_program) = ColorView::initialize_programs(&context);

                let camera = Camera::new_perspective(
                    Viewport::new_at_origo(1, 1),
                    vec3(0.0, 2.0, 4.0),
                    vec3(0.0, 0.5, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    degrees(45.0),
                    0.1,
                    10.0,
                );

                let color_scene = Scene::new(&context, include_str!("hsv.frag"));
                let pos_scene = Scene::new(&context, include_str!("pos.frag"));
                let models = vec![
                    Model { Vertex }
                ]
                let view = ColorView { window, selection: vec3(0.0, 0.0, 0.0), on_select: None, on_hover: None, camera, color_scene, models };
                view.render_loop();
                view
            },
            _ => panic!("ColorView::new must be passed a canvas!")
        }
    }

    fn initialize_models(context: &Context) -> Vec<Model> {
        let (positions, colors) = polar_cylinder(64);
        let cylinder = VertexBuffer::new_with_data(
            &context,
            &positions
        );
        let cube = VertexBuffer::new_with_data(
            &context,
            &CpuMesh::cube().positions.to_f32(),
        );
        vec![cylinder, cube]
    }

    pub fn render_loop(&self) {

    }
}

#[wasm_bindgen]
pub fn new_canvas(canvas: HtmlCanvasElement, width: u32, height: u32, callback: js_sys::Function) {
    console_error_panic_hook::set_once();
    let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
        Ok(canvas) => {
            Ok(canvas)
        }
        Err(_) => Err(JsValue::from_str("Fail! Not a canvas!"))
    }.unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("winit window")
        .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_maximized(true);
    #[cfg(target_arch = "wasm32")]
    let window_builder = {
        use winit::platform::web::WindowBuilderExtWebSys;
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                canvas
            ))
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            // .with_prevent_default(true)
    };

    let event_loop = winit::event_loop::EventLoop::new();
    let winit_window = window_builder.build(&event_loop).unwrap();
    // let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();
    let window = Window::from_winit_window(winit_window, event_loop, SurfaceSettings::default(), false).unwrap();
    let context = window.gl();
    let program = Program::from_source(
        &context, 
        include_str!("color.vert"),
        include_str!("hsv.frag"),
    ).unwrap();

    let depth_program = Program::from_source(
        &context, 
        include_str!("color.vert"),
        include_str!("pos.frag"),
    ).unwrap();

    // Create camera
    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 2.0, 4.0),
        vec3(0.0, 0.5, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let (positions, colors) = polar_cylinder(64);
    let positions = VertexBuffer::new_with_data(
        &context,
        &positions
    );
    let cube_positions = VertexBuffer::new_with_data(
        &context,
        &CpuMesh::cube().positions.to_f32(),
    );
    let render_states = RenderStates {
        depth_test: DepthTest::Less,
        blend: Blend::TRANSPARENCY,
        ..Default::default()
    };
    let mut screen_position = Box::new(Vec2::new(0.0, 0.0));
    // Event loop
    // let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    window.render_loop(move |mut frame_input| {
        let mut texture = Texture2D::new_empty::<[f32; 4]>(
            &context,
            width,
            height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let mut depth_texture = DepthTexture2D::new::<f32>(
            &context,
            width,
            height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let depth_target = RenderTarget::new(
            texture.as_color_target(None),
            depth_texture.as_depth_target(),
        );
        control.handle_events(&mut camera, &mut frame_input.events);
        let mut position: Option<Vector2<f32>> = None;
        for event in frame_input.events.iter() {
            match event {
                Event::MouseMotion {position: pos, ..} => {
                    position = Some(Vec2::new(pos.x, pos.y));
                }
                _ => {}
            }
        }
        camera.set_viewport(frame_input.viewport);

        let u_model = Mat4::from_translation(Vector3::new(0.0, 1.0, 0.0)) *
            Mat4::from_angle_z(degrees(-90.0));

        let u_view = camera.projection() * camera.view();

        let scissor_box = ScissorBox { x: screen_position.x as i32, y: (height as i32)-(screen_position.y as i32), width: 1, height: 1 };
        let world_position = depth_target
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))
            .write(|| {
                depth_program.use_uniform(
                    "model",
                    u_model
                );
                depth_program.use_uniform(
                    "view",
                    u_view
                );
                depth_program.use_vertex_attribute("position", &positions);
                depth_program.draw_arrays(RenderStates::default(), frame_input.viewport, positions.vertex_count());
            })
            .read_color_partially::<[f32; 4]>(scissor_box)[0];
        let hit = world_position[3] == 1.0;
        let world_position = Vec3::new(
            world_position[0],
            world_position[1],
            world_position[2],
        );

        // model.animate(frame_input.accumulated_time as f32);
        let screen = frame_input
            .screen();
        let target = screen
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.0, 1.0))
            .write(|| {
                program.use_uniform(
                    "model",
                    u_model
                );
                program.use_uniform(
                    "view",
                    u_view
                );
                program.use_vertex_attribute("position", &positions);
                program.draw_arrays(RenderStates::default(), frame_input.viewport, positions.vertex_count());
            });
        if let Some(position) = position {
            *screen_position = Vec2::new(position.x, position.y);
            if hit {
                let color = target.read_color_partially::<[u8; 4]>(
                    scissor_box
                )[0];
                let this = JsValue::null();
                let _ = callback.call3(
                    &this,
                    &JsValue::from_f64(color[0] as f64 / 255.0), 
                    &JsValue::from_f64(color[1] as f64 / 255.0), 
                    &JsValue::from_f64(color[2] as f64 / 255.0),
                );
            }
        }
        screen
            .write(|| {
                program.use_uniform(
                    "model",
                    Mat4::from_translation(world_position) *
                    Mat4::from_scale(0.2)
                );
                program.use_uniform(
                    "view",
                    u_view
                );
                program.use_vertex_attribute("position", &cube_positions);
                program.draw_arrays(render_states, frame_input.viewport, cube_positions.vertex_count())
            });

        FrameOutput::default()
    });
}