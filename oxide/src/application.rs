use crate::log;
use crate::window::{run_with_handler, WindowHandler};

pub struct Application {
    title: String,
}

impl Application {
    pub fn new() -> Self {
        log::info!("Oxide Application created!");
        Self {
            title: "Oxide Window".to_string(),
        }
    }

    pub fn run(&mut self) {
        log::info!("Oxide Application running...");
        let handler = WindowHandler::new(&self.title);
        // Blocks until exit (e.g., on CloseRequested)
        run_with_handler(handler).expect("Event loop failed");
        log::info!("Oxide Application shut down.");
    }
}
