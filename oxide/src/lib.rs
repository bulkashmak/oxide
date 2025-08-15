pub mod log;

pub struct Application;

impl Application {
    pub fn new() -> Self {
        log::init();
        log::info!("Oxide Application created!");
        Self
    }

    pub fn run(&self) {
        log::info!("Oxide Application running...");
    }
}
