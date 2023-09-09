extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use three_d::{renderer::*, FrameInputGenerator, SurfaceSettings, WindowedContext};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

struct MandelbrotMaterial {}

impl Material for MandelbrotMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("mandelbrot.frag").to_string()
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

#[wasm_bindgen]
pub fn new_canvas(canvas: HtmlCanvasElement, width: u32, height: u32) {
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
            .with_prevent_default(true)
    };

    let event_loop = winit::event_loop::EventLoop::new();
    let window = window_builder.build(&event_loop).unwrap();
    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    // Create camera
    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 2.0, 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // let positions = CpuMesh::cone(4).positions;
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let subdivisions = 16;
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
    // let colors = positions.iter().map(|x| Srgba::new_opaque(
    //     (x.x * 255.0) as u8,
    //     (x.y * 255.0) as u8,
    //     (x.z * 255.0) as u8,))
    //     .collect::<Vec<Srgba>>();
    let positions = Positions::F32(positions);
    let mesh = CpuMesh {
        positions,
        // colors: Some(colors),
        ..Default::default()
    };
    // let material = ColorMaterial {
    //     render_states: RenderStates {
    //         cull: Cull::None,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };
    let material = MandelbrotMaterial {};
    // Create model
    let mut model = Gm::new(
        Mesh::new(&context, &mesh),
        material,
    );
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.0002)));

    // Event loop
    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    event_loop.spawn(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let mut frame_input = frame_input_generator.generate(&context);

            control.handle_events(&mut camera, &mut frame_input.events);
            camera.set_viewport(frame_input.viewport);
            model.animate(frame_input.accumulated_time as f32);
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(&camera, &model, &[]);

            context.swap_buffers().unwrap();
            control_flow.set_poll();
            window.request_redraw();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            frame_input_generator.handle_winit_window_event(event);
            // match event {
            //     winit::event::WindowEvent::Resized(physical_size) => {
            //         context.resize(*physical_size);
            //     }
            //     winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            //         context.resize(**new_inner_size);
            //     }
            //     winit::event::WindowEvent::CloseRequested => {
            //         control_flow.set_exit();
            //     }
            //     _ => (),
            // }
        }
        _ => {}
    });
}