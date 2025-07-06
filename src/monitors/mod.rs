pub mod nvidia;
pub mod amd;
pub mod intel;
pub mod apple;
pub mod generic;

use crate::hardware_detection::{HardwareMonitor, HardwareInfo};
use crate::model::SharedAppState;

pub struct MonitorRegistry {
    monitors: Vec<Box<dyn HardwareMonitor>>,
}

impl MonitorRegistry {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }
    
    pub fn register_all_monitors(&mut self) {
        // Register all available monitors
        self.monitors.push(Box::new(nvidia::NvidiaMonitor::new()));
        self.monitors.push(Box::new(amd::AmdMonitor::new()));
        self.monitors.push(Box::new(intel::IntelMonitor::new()));
        self.monitors.push(Box::new(apple::AppleMonitor::new()));
        self.monitors.push(Box::new(generic::GenericMonitor::new()));
    }
    
    pub fn initialize_for_hardware(&mut self, hardware_info: &HardwareInfo) -> Result<(), Box<dyn std::error::Error>> {
        for monitor in &mut self.monitors {
            if monitor.supports_hardware(hardware_info) {
                if let Err(e) = monitor.initialize() {
                    crate::logger::log_error(&format!("Failed to initialize monitor: {}", e), &*e);
                }
            }
        }
        Ok(())
    }
    
    pub fn update_all_metrics(&mut self, state: &SharedAppState) -> Result<(), Box<dyn std::error::Error>> {
        for monitor in &mut self.monitors {
            if let Err(e) = monitor.update_metrics(state) {
                crate::logger::log_error(&format!("Monitor update failed: {}", e), &*e);
            }
        }
        Ok(())
    }
}