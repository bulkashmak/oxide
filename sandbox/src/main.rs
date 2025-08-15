use oxide::{Application, AppCreator, run_game};

struct SandboxApp;

impl AppCreator for SandboxApp {
    fn create_application() -> Application {
        Application::new()
    }
}

fn main() {
    run_game::<SandboxApp>();
}
