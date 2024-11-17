use winit_classic::{WinitContext, WinitEvent, WinitHandler};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

fn main() {
    WinitContext::start(winit_main).expect("An event loop error occured!");
}

fn winit_main(window: &Window, handler: &mut WinitHandler) {
    let mut cache = String::new();
    let mut count = 0usize;

    handler.on_window_event(move |event_loop: &ActiveEventLoop, event: WinitEvent|{
        match event {
            WinitEvent::WindowEvent(window_id, window_event) => {
                match window_event {
                    WindowEvent::CursorEntered { device_id } => {},
                    WindowEvent::CursorEntered { device_id } => {}
                    _ => {}
                } 
            },
            _ => {}
        }
    });
}