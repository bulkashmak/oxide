use oxide::Application;

fn main() {
    oxide::log::info!("Sandbox starting...");

    let mut app = Application::new();
    
    // Run in a separate thread for demonstration
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
        // We'll stop after 3 seconds
        // In future this will come from events
        println!("Requesting shutdown...");
        // We can't stop here yet without shared mutability; 
        // will handle with proper events later
    });

    app.run();
}
