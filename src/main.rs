use std::panic;
use std::time::{Duration, Instant};
use console_error_panic_hook;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
    dpi::LogicalSize,
};

use pixels::{
    Pixels,
    SurfaceTexture,
    wgpu::Color,
};

mod ui;

#[cfg(target_arch = "x86_64")]
mod pixels_draw_target;

#[cfg(target_arch = "wasm32")]
mod canvas_display;

use embedded_graphics::{
    prelude::*,
    primitives::{Circle, Rectangle, PrimitiveStyleBuilder},
    text::Text,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
}; 

// this is pub because it's called from the wasm module below
pub fn main() {
    println!("Welcome to Carsten's winit testing program.");

    #[cfg(target_arch = "wasm32")]
    {
        // with this we can get panic messages in the console if targeting wasm
        panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    println!("2");
    let el = EventLoop::new();
    // use winit_input_helper::WinitInputHelper;
    // let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Carsten's aweseome winit/wasm window")
        .with_inner_size(LogicalSize::new(256, 256))
        .build(&el)
        .unwrap();

    log::info!("Window size is {} x {}", window.inner_size().width, window.inner_size().height);

    let graphics = ui::draw_text();
    /**************************************************************************
    This is the setup if compiling for x86_64 machines
    **************************************************************************/
    #[cfg(target_arch = "x86_64")]
    {
        use pixels_draw_target::Pixelbuffer;
        let mut pixel_buffer = Pixelbuffer::new(window);
        let start = Instant::now();
        graphics.draw(&mut pixel_buffer).unwrap();
        let duration = start.elapsed();
        println!("Time elapsed in drawing graphics is: {:?}", duration);
    }

    /**************************************************************************
    This is the setup if compiling for web assembly
    **************************************************************************/
    #[cfg(target_arch = "wasm32")]
    {   // here we initialize the wasm stuff
        use winit::platform::web::WindowExtWebSys;
        use canvas_display::CanvasDisplay;

        let canvas   = window.canvas();
        let size     = window.inner_size();
        let window   = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body     = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Error appending canvas to HTML body :(");

        let mut pixel_buffer = CanvasDisplay::new(&canvas);
        log::info!("CanvasDisplay size is {} x {}", pixel_buffer.width, pixel_buffer.height);
        graphics.draw(&mut pixel_buffer).unwrap();
    }

    /**************************************************************************
    Now we run the event loop closure
    **************************************************************************/
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[cfg(target_arch = "wasm32")]
        // log::debug!("{:?}", event);

        #[cfg(target_arch = "x86_64")]
        // println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::MouseInput {..},
                window_id,
            } => log::debug!("{:?}", event),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
        // window.request_redraw();
    });
    
}

/******************************************************************************
When building for web assembly, call the main function after setting up other stuff
******************************************************************************/
#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use futures::executor::block_on;

    #[wasm_bindgen(start)]
    pub fn wasm_start() {
        console_log::init_with_level(log::Level::Debug).expect("Error initializing logger!");
        // run the program entry point from here
        log::info!("Running program entry point...");
        super::main();
    }

}
