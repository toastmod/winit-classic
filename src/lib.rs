
use winit::{application::ApplicationHandler, error::EventLoopError, event::{DeviceEvent, DeviceId, StartCause, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


pub enum WinitEvent {
    WindowEvent(WindowId, WindowEvent),
    DeviceEvent(DeviceId, DeviceEvent),
    AboutToWait,
    MemoryWarning,
    Exiting,
    NewEvents(StartCause),
    Suspended

}

pub struct WinitContext {
    mainf: fn(&Window, &mut WinitHandler) -> (),
}

impl WinitContext {
    pub fn start(mainf: fn(&Window, &mut WinitHandler) -> ()) -> Result<(), EventLoopError> {
        let mut context = Self {
            mainf
        };

        let mut event_loop = EventLoop::new()?;

        event_loop.run_app(&mut WinitHandler {
            context: &mut context,
            window_event_f: Box::new(|_,_|{}),
        })?;

        Ok(())

    }
}


pub struct WinitHandler<'context> {
    context: &'context mut WinitContext,
    window_event_f: Box<dyn FnMut(&ActiveEventLoop, WinitEvent) + 'context>,
}

impl<'context> WinitHandler<'context> {
    pub fn on_window_event<F: FnMut(&ActiveEventLoop, WinitEvent) + 'context >(&mut self, f: F) {
        self.window_event_f = Box::new(f);
    }

}

impl<'context> ApplicationHandler for WinitHandler<'context> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut window = event_loop.create_window(Default::default()).expect("Couldn't create window!");
        (self.context.mainf)(&window, self)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // self.window_event_f(event_loop, window_id, event, self)
        (self.window_event_f)(event_loop, WinitEvent::WindowEvent(window_id, event))
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        (self.window_event_f)(event_loop, WinitEvent::AboutToWait)
    }

    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        (self.window_event_f)(event_loop, WinitEvent::DeviceEvent(device_id, event))
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        (self.window_event_f)(event_loop, WinitEvent::Exiting)
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        (self.window_event_f)(event_loop, WinitEvent::MemoryWarning)
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        (self.window_event_f)(event_loop, WinitEvent::NewEvents(cause))
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        (self.window_event_f)(event_loop, WinitEvent::Suspended)
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        // (self.window_event_f)(event_loop, WinitEvent::Suspended)
        //TODO: User defined events
    }
}



