use oxide::Application;

fn main() {
    oxide::log::info!("Sandbox starting...");

    let app = Application::new();
    app.run();
}
