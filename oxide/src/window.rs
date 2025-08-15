use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId, WindowAttributes},
};

pub struct WindowHandler {
    title: String,
    window: Option<Window>,
}

impl WindowHandler {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            window: None,
        }
    }
}

impl ApplicationHandler<()> for WindowHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(800.0, 600.0));

        let window = event_loop.create_window(attrs).expect("Failed to create window");

        // Request at least one redraw so the compositor maps the surface
        window.request_redraw();

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                // Normally you’d render here — for now, just trigger mapping
                log::info!("Redraw requested");
            }
            WindowEvent::CloseRequested => {
                log::info!("Close requested, exiting…");
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

pub fn run_with_handler(mut handler: WindowHandler) -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut handler)
}
