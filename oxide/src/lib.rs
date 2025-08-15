pub mod log;

pub struct Application {
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        log::init();
        log::info!("Oxide Application created!");
        Self { running: true }
    }

    pub fn run(&mut self) {
        log::info!("Oxide Application starting main loop...");

        while self.running {
            self.update();
        }
    }

    fn update(&self) {
        log::debug!("Application update tick...");
        // Simulate doing work
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    pub fn stop(&mut self) {
        log::info!("Stopping Oxide Application...");
        self.running = false;
    }
}
