# Winit Classic
A temporary solution for moving from crate version `0.29` to `0.30`.

## Example
```rust
use winit_classic::{WinitContext, WinitEvent, WinitHandler};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

fn main() {
    // Create the context, providing a function that is called when the window is created.
    WinitContext::start(winit_main).expect("An event loop error occured!");
}

// Define the window startup function
fn winit_main(window: &Window, handler: &mut WinitHandler) {

    // Define the window event handler
    // `WinitEvent` recreates the usual `match` pattern from `0.29` 
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
```
