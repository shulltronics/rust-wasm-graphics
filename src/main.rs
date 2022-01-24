use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

mod ui;

// this is pub because it's called from the wasm module below
pub fn main() {
    //println!("Welcome to Carsten's winit testing program.");

    ui::draw_text();

    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Carsten's aweseome winit/wasm window")
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

    #[wasm_bindgen(start)]
    pub fn wasm_start() {
        console_log::init_with_level(log::Level::Debug).expect("Error initializing logger!");
        // run the program entry point from here
        super::main();
    }

}
