extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use three_d::{renderer::*, FrameInputGenerator, SurfaceSettings, WindowedContext};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn new_canvas(canvas: HtmlCanvasElement, width: u32, height: u32) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
        Ok(canvas) => {
            Ok(canvas)
        }
        Err(_) => Err(JsValue::from_str("Fail! Not a canvas!"))
    }.unwrap();
    let window_builder = {
        use winit::platform::web::WindowBuilderExtWebSys;
        // let (w, h) = (canvas.width(), canvas.height());
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                canvas
            ))
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
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

    // Create model
    let mut model = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::GREEN,
            ..Default::default()
        },
    );
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.0005)));

    // Event loop
    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    event_loop.run(move |event, _, control_flow| match event {
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
        // winit::event::Event::WindowEvent { ref event, .. } => {
        //     frame_input_generator.handle_winit_window_event(event);
        //     match event {
        //         winit::event::WindowEvent::Resized(physical_size) => {
        //             context.resize(*physical_size);
        //         }
        //         winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        //             context.resize(**new_inner_size);
        //         }
        //         winit::event::WindowEvent::CloseRequested => {
        //             control_flow.set_exit();
        //         }
        //         _ => (),
        //     }
        // }
        _ => {}
    });
}