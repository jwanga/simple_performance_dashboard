use std::thread;
use std::time::Duration;
use crate::model::SharedAppState;
use crate::logger;
use crate::hardware_detection::{HardwareDetector, HardwareInfo};
use crate::monitors::MonitorRegistry;

pub struct HardwarePoller {
    state: SharedAppState,
    polling_interval: Duration,
    hardware_info: HardwareInfo,
    monitor_registry: MonitorRegistry,
}

impl HardwarePoller {
    pub fn new(state: SharedAppState, polling_interval_ms: u64) -> Self {
        let hardware_info = HardwareDetector::detect();
        let mut monitor_registry = MonitorRegistry::new();
        monitor_registry.register_all_monitors();
        
        logger::log_info(&format!("Detected hardware: CPU={:?}, GPUs={:?}, Platform={:?}", 
            hardware_info.cpu_vendor, 
            hardware_info.gpu_vendors, 
            hardware_info.platform
        ));
        
        Self {
            state,
            polling_interval: Duration::from_millis(polling_interval_ms),
            hardware_info,
            monitor_registry,
        }
    }
    
    pub fn start_polling_thread(mut self) -> thread::JoinHandle<()> {
        // Initialize monitors for detected hardware
        if let Err(e) = self.monitor_registry.initialize_for_hardware(&self.hardware_info) {
            logger::log_error("Failed to initialize hardware monitors", &*e);
        }
        
        thread::spawn(move || {
            loop {
                self.poll_hardware();
                thread::sleep(self.polling_interval);
            }
        })
    }
    
    pub fn poll_hardware(&mut self) {
        // Update all metrics using the monitor registry
        if let Err(e) = self.monitor_registry.update_all_metrics(&self.state) {
            logger::log_error("Failed to update hardware metrics", &*e);
        }
    }
}

// Re-export the error type for backward compatibility
pub use crate::hardware_detection::HardwareMonitor;

#[derive(Debug)]
pub enum HardwareError {
    SensorUnavailable(String),
    ReadFailure(String),
    InitializationFailed(String),
}

impl std::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareError::SensorUnavailable(msg) => write!(f, "Sensor unavailable: {}", msg),
            HardwareError::ReadFailure(msg) => write!(f, "Read failure: {}", msg),
            HardwareError::InitializationFailed(msg) => write!(f, "Initialization failed: {}", msg),
        }
    }
}

impl std::error::Error for HardwareError {}

pub type HardwareResult<T> = Result<T, HardwareError>;