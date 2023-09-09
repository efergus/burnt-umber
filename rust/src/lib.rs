extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use three_d::{renderer::{*, control::Event}, SurfaceSettings, Window, FrameInput, FrameOutput};
// use 

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct MandelbrotMaterial {}

impl Material for MandelbrotMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("hsv.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {}
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            write_mask: WriteMask::COLOR,
            cull: Cull::Back,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn id(&self) -> u16 {
        0b11u16
    }
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
    // Event loop
    // let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    window.render_loop(move |mut frame_input| {
        control.handle_events(&mut camera, &mut frame_input.events);
        let mut position: Option<Vector2<f32>> = None;
        for event in frame_input.events.iter() {
            match event {
                Event::MouseMotion {position: pos, ..} => {
                    position = Some(Vec2::new(pos.x, pos.y))
                }
                _ => {}
            }
        }
        camera.set_viewport(frame_input.viewport);

        // model.animate(frame_input.accumulated_time as f32);
        let screen = frame_input
            .screen();
        let target = screen
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.0, 1.0))
            .write(|| {
                program.use_uniform(
                    "model",
                    Mat4::from_translation(Vector3::new(0.0, 1.0, 0.0)) *
                    Mat4::from_angle_z(degrees(-90.0))
                );
                program.use_uniform(
                    "view",
                    camera.projection() * camera.view()
                );
                program.use_vertex_attribute("position", &positions);
                program.draw_arrays(RenderStates::default(), frame_input.viewport, positions.vertex_count())
            });
        if let Some(position) = position {
            let color = target.read_color_partially::<[u8; 4]>(
                ScissorBox { x: position.x as i32, y: (height as i32)-(position.y as i32), width: 1, height: 1 }
            )[0];
            // log(&format!("{:?}", color));
            let this = JsValue::null();
            let _ = callback.call3(
                &this,
                &JsValue::from_f64(color[0] as f64 / 255.0), 
                &JsValue::from_f64(color[1] as f64 / 255.0), 
                &JsValue::from_f64(color[2] as f64 / 255.0),
            );
        }

        // context.swap_buffers().unwrap();
        // control_flow.set_poll();
        // window.request_redraw();
        FrameOutput::default()
    });
}