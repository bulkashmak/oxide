use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

/// Simple wrapper over a winit Window + title config.
pub struct AppWindow {
    pub window: Window,
}

impl AppWindow {
    fn create(event_loop: &ActiveEventLoop, title: &str) -> Self {
        let attrs = WindowAttributes::default().with_title(title.to_string());
        let window = event_loop
            .create_window(attrs)
            .expect("Failed to create window");
        crate::log::info!("Window created: \"{}\"", title);
        Self { window }
    }
}

/// The engine's application handler that owns the window during the event loop.
pub struct WindowHandler {
    title: String,
    window: Option<AppWindow>,
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
        self.window = Some(AppWindow::create(event_loop, &self.title));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            crate::log::info!("Window close requested. Exiting event loop...");
            event_loop.exit();
        }
    }
}

/// Run a windowed app with the given handler.
pub fn run_with_handler(mut handler: WindowHandler) -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut handler)
}
