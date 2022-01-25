use std::panic;
use console_error_panic_hook;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
    dpi::LogicalSize,
};

// TODO: move ui logic to ui module
use pixels::{
    Pixels,
    SurfaceTexture,
    wgpu::Color,
};

mod ui;
mod canvas_display;
use canvas_display::CanvasDisplay;
use embedded_graphics::{
    prelude::*,
    primitives::{Circle, Rectangle, PrimitiveStyleBuilder},
    text::Text,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
}; 

// this is pub because it's called from the wasm module below
pub async fn main() {
    //println!("Welcome to Carsten's winit testing program.");

    #[cfg(target_arch = "wasm32")]
    {
        // with this we can get panic messages in the console if targeting wasm
        panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    ui::draw_text();

    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Carsten's aweseome winit/wasm window")
        .with_inner_size(LogicalSize::new(256, 256))
        .build(&el)
        .unwrap();

    log::info!("Window size is {} x {}", window.inner_size().width, window.inner_size().height);

    #[cfg(target_arch = "wasm32")]
    {   // here we initialize the wasm stuff
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Error appending canvas to HTML body :(");

        let mut cd = CanvasDisplay::new(canvas);
        log::info!("CanvasDisplay size is {} x {}", cd.width, cd.height);

        let circle = {
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::RED)
                .stroke_width(1)
                .fill_color(Rgb888::GREEN)
                .build();
            Circle::new(Point::new(10, 10), 50)
                .into_styled(style)
        };

        let text = {
            let style = MonoTextStyle::new(&FONT_10X20, Rgb888::new(255, 255, 0));
            Text::new("hello Web Assembly!", Point::new(0, 0), style)
        };

        circle.draw(&mut cd).unwrap();

    }

    // now we run the event loop closure
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[cfg(target_arch = "wasm32")]
        //log::debug!("{:?}", event);

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
    });
    
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use futures::executor::block_on;

    #[wasm_bindgen(start)]
    pub fn wasm_start() {
        console_log::init_with_level(log::Level::Debug).expect("Error initializing logger!");
        // run the program entry point from here
        log::info!("Running program entry point...");
        block_on(super::main());
    }

}
