mod model;
mod hardware;
mod ui;
mod logger;

use model::AppState;
use hardware::HardwarePoller;
use ui::run_app;

fn main() -> eframe::Result<()> {
    // Initialize logging system
    if let Err(e) = logger::initialize_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }
    
    logger::log_info("Simple Performance Dashboard starting...");
    
    // Initialize shared application state
    let polling_interval_ms = 1000; // 1 second default
    let app_state = AppState::new_shared(polling_interval_ms);
    
    logger::log_info(&format!("Initialized application state with {}ms polling interval", polling_interval_ms));
    
    // Start hardware polling thread
    let poller = HardwarePoller::new(app_state.clone(), polling_interval_ms);
    let _polling_handle = poller.start_polling_thread();
    
    logger::log_info("Hardware polling thread started");
    
    // Run the GUI application
    logger::log_info("Starting GUI application");
    run_app(app_state)
}
