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

    //ui::draw_text();
    let el = EventLoop::new();
    // use winit_input_helper::WinitInputHelper;
    // let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Carsten's aweseome winit/wasm window")
        .with_inner_size(LogicalSize::new(256, 256))
        .build(&el)
        .unwrap();

    println!("Window size is {} x {}", window.inner_size().width, window.inner_size().height);
    log::info!("Window size is {} x {}", window.inner_size().width, window.inner_size().height);

    /**************************************************************************
    This is the setup if compiling for x86_64 machines
    **************************************************************************/
    //#[cfg(target_arch = "x86_64")]
    //{
    use pixels_draw_target::Pixelbuffer;
    let mut pb = Pixelbuffer::new(&window);

    let square = {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::GREEN)
            .build();
        Rectangle::new(Point::new(100, 100), Size::new(5, 5))
            .into_styled(style)
    };

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
        let style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);
        Text::new("hello Web Assembly!", Point::new(0, 15), style)
    };

    println!("  starting direct buffer draw...");
    let time: Instant = Instant::now();
    //    square.draw(&mut pb).unwrap();
    square.draw(&mut pb).unwrap();
    println!("  draw time for square: {:?}", time.elapsed());
    circle.draw(&mut pb).unwrap();
    text.draw(&mut pb).unwrap();


    /**************************************************************************
    This is the setup if compiling for web assembly
    **************************************************************************/
   //#[cfg(target_arch = "wasm32")]
   //{   // here we initialize the wasm stuff
   //    use winit::platform::web::WindowExtWebSys;
   //    use canvas_display::CanvasDisplay;

   //    let canvas = window.canvas();
   //    let size = window.inner_size();
   //    let window = web_sys::window().unwrap();
   //    let document = window.document().unwrap();
   //    let body = document.body().unwrap();

   //    body.append_child(&canvas)
   //        .expect("Error appending canvas to HTML body :(");

   //    let mut cd = CanvasDisplay::new(&canvas);
   //    log::info!("CanvasDisplay size is {} x {}", cd.width, cd.height);

   //    let circle = {
   //        let style = PrimitiveStyleBuilder::new()
   //            .stroke_color(Rgb888::RED)
   //            .stroke_width(1)
   //            .fill_color(Rgb888::GREEN)
   //            .build();
   //        Circle::new(Point::new(10, 10), 50)
   //            .into_styled(style)
   //    };

   //    let text = {
   //        let style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLACK);
   //        Text::new("hello Web Assembly!", Point::new(0, 15), style)
   //    };

   //    circle.draw(&mut cd).unwrap();
   //    text.draw(&mut cd).unwrap();

   //}

    /**************************************************************************
    Now we run the event loop closure
    **************************************************************************/
    el.run(move |event, _, control_flow| {
        //*control_flow = ControlFlow::Wait;
        control_flow.set_poll();
        control_flow.set_wait();

        //#[cfg(target_arch = "wasm32")]
        // log::debug!("{:?}", event);

        //#[cfg(target_arch = "x86_64")]
        //println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                println!("App: closing window.");
                control_flow.set_exit();
            },
            Event::WindowEvent {
                event: WindowEvent::MouseInput {..},
                window_id,
            } => {
                if window_id == window.id() {
                    println!("{:?}", event);
                }
                //window.request_redraw();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                let time = Instant::now();
                pb.render();
                println!("  rendering time: {:?}", time.elapsed());
            },
            _ => (),
        }

    }); // event loop closure

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

