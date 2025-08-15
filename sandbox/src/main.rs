use oxide::{log, Application};

fn main() {
    log::init();

    let mut app = Application::new();
    app.run();
}
