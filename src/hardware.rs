use std::thread;
use std::time::Duration;
use sysinfo::{System, Components};
use crate::model::SharedAppState;
use crate::logger;

pub struct HardwarePoller {
    state: SharedAppState,
    system: System,
    components: Components,
    polling_interval: Duration,
}

impl HardwarePoller {
    pub fn new(state: SharedAppState, polling_interval_ms: u64) -> Self {
        Self {
            state,
            system: System::new_all(),
            components: Components::new_with_refreshed_list(),
            polling_interval: Duration::from_millis(polling_interval_ms),
        }
    }
    
    pub fn start_polling_thread(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                self.poll_hardware();
                thread::sleep(self.polling_interval);
            }
        })
    }
    
    pub fn poll_hardware(&mut self) {
        // Refresh system information
        self.system.refresh_all();
        self.components.refresh();
        
        // Update CPU metrics
        if let Err(e) = self.update_cpu_metrics() {
            logger::log_error("Failed to update CPU metrics", &e);
        }
        
        // Update GPU metrics (limited support in sysinfo)
        if let Err(e) = self.update_gpu_metrics() {
            logger::log_error("Failed to update GPU metrics", &e);
        }
        
        // Update memory metrics
        if let Err(e) = self.update_memory_metrics() {
            logger::log_error("Failed to update memory metrics", &e);
        }
        
        // Update storage metrics
        if let Err(e) = self.update_storage_metrics() {
            logger::log_error("Failed to update storage metrics", &e);
        }
        
        // Update motherboard metrics (temperatures, fans)
        if let Err(e) = self.update_motherboard_metrics() {
            logger::log_error("Failed to update motherboard metrics", &e);
        }
    }
    
    fn update_cpu_metrics(&mut self) -> HardwareResult<()> {
        let mut state = self.state.write();
        
        // CPU utilization (average across all cores)
        let cpu_usage = self.system.global_cpu_usage();
        state.cpu.utilization.update(cpu_usage);
        
        // CPU frequency (from first core as representative)
        if let Some(cpu) = self.system.cpus().first() {
            let frequency_mhz = cpu.frequency() as u32;
            if frequency_mhz > 0 {
                state.cpu.clock_speed.update(frequency_mhz);
            }
        }
        
        // CPU temperature (from components)
        let cpu_temp = self.get_cpu_temperature();
        if let Some(temp) = cpu_temp {
            state.cpu.package_temperature.update(temp);
        }
        
        // Note: Core voltage, power consumption, thermal throttling and hotspot temperature 
        // require more advanced APIs (like Intel Power Gadget, AMD Ryzen Master APIs, etc.)
        // For now, we'll leave these as placeholder implementations
        // In a production system, you'd integrate with platform-specific libraries
        
        Ok(())
    }
    
    fn update_gpu_metrics(&mut self) -> HardwareResult<()> {
        // GPU metrics are limited in sysinfo
        // For comprehensive GPU monitoring, we'd need GPU-specific libraries
        // like NVML for NVIDIA or ADL for AMD
        // For now, we'll implement basic placeholders
        
        let mut state = self.state.write();
        
        // GPU temperature might be available through components
        let gpu_temp = self.get_gpu_temperature();
        if let Some(temp) = gpu_temp {
            state.gpu.package_temperature.update(temp);
        }
        
        // Note: GPU utilization, clock speed, memory utilization, voltage, power consumption
        // require GPU-specific APIs (NVML, ADL, etc.)
        // In a production system, you'd integrate with vendor-specific SDKs
        
        Ok(())
    }
    
    fn update_memory_metrics(&mut self) -> HardwareResult<()> {
        let mut state = self.state.write();
        
        // Memory utilization in MB
        let used_memory = self.system.used_memory();
        let usage_mb = (used_memory / 1024 / 1024) as u64;
        state.memory.utilization_mb.update(usage_mb);
        
        // Memory temperature might be available through components
        let memory_temp = self.get_memory_temperature();
        if let Some(temp) = memory_temp {
            state.memory.temperature.update(temp);
        }
        
        // Note: Memory clock speed requires platform-specific APIs
        // (DMI/SMBIOS access, manufacturer-specific tools, etc.)
        
        Ok(())
    }
    
    fn update_storage_metrics(&mut self) -> HardwareResult<()> {
        let mut state = self.state.write();
        
        // Storage metrics (disk I/O, temperatures) are not readily available through sysinfo
        // For comprehensive storage monitoring, platform-specific APIs would be needed:
        // - Windows: Performance Counters, WMI, SMART data access
        // - Linux: /proc/diskstats, SMART tools, sysfs
        // - Cross-platform: Third-party libraries like libatasmart
        
        // Note: Drive read/write speeds and temperatures require specialized libraries
        // In a production system, you'd integrate with storage monitoring APIs
        
        Ok(())
    }
    
    fn update_motherboard_metrics(&mut self) -> HardwareResult<()> {
        let mut state = self.state.write();
        
        // Temperature sensors
        for component in &self.components {
            let temp = component.temperature();
            let label = component.label().to_lowercase();
            
            // Categorize temperatures based on updated requirements
            if label.contains("chipset") || label.contains("motherboard") {
                state.motherboard.chipset_temperature.update(temp);
            } else if label.contains("chassis") || label.contains("case") || label.contains("system") {
                state.motherboard.chassis_temperature.update(temp);
            }
        }
        
        // Fan speeds - sysinfo has limited support for fans
        // For comprehensive fan monitoring (AIO pump, chassis fans, chipset fans),
        // platform-specific implementations would be needed:
        // - Windows: WMI, manufacturer SDKs (like iCUE, NZXT CAM APIs)
        // - Linux: hwmon, lm-sensors
        
        Ok(())
    }
    
    fn get_cpu_temperature(&self) -> Option<f32> {
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("cpu") && (label.contains("package") || label.contains("tctl") || label.contains("tdie")) {
                return Some(component.temperature());
            }
        }
        // Fallback to first CPU-related temperature
        for component in &self.components {
            if component.label().to_lowercase().contains("cpu") {
                return Some(component.temperature());
            }
        }
        None
    }
    
    fn get_gpu_temperature(&self) -> Option<f32> {
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("gpu") || label.contains("graphics") || label.contains("video") {
                return Some(component.temperature());
            }
        }
        None
    }
    
    fn get_memory_temperature(&self) -> Option<f32> {
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("memory") || label.contains("ram") || label.contains("dimm") {
                return Some(component.temperature());
            }
        }
        None
    }
}

// Error handling for hardware polling
#[derive(Debug)]
pub enum HardwareError {
    SensorUnavailable(String),
    ReadFailure(String),
}

impl std::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareError::SensorUnavailable(sensor) => write!(f, "Sensor unavailable: {}", sensor),
            HardwareError::ReadFailure(error) => write!(f, "Read failure: {}", error),
        }
    }
}

impl std::error::Error for HardwareError {}

pub type HardwareResult<T> = Result<T, HardwareError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AppState;
    use std::sync::Arc;
    use parking_lot::RwLock;
    use std::time::Duration;

    fn create_test_state() -> SharedAppState {
        Arc::new(RwLock::new(AppState::new(100))) // 100ms for faster testing
    }

    #[test]
    fn test_hardware_poller_creation() {
        let state = create_test_state();
        let poller = HardwarePoller::new(state.clone(), 500);
        
        // Verify polling interval is set correctly
        assert_eq!(poller.polling_interval, Duration::from_millis(500));
    }

    #[test]
    fn test_hardware_error_display() {
        let sensor_error = HardwareError::SensorUnavailable("CPU Temperature".to_string());
        let read_error = HardwareError::ReadFailure("Permission denied".to_string());
        
        assert_eq!(format!("{}", sensor_error), "Sensor unavailable: CPU Temperature");
        assert_eq!(format!("{}", read_error), "Read failure: Permission denied");
    }

    #[test]
    fn test_hardware_error_debug() {
        let sensor_error = HardwareError::SensorUnavailable("GPU Clock".to_string());
        let debug_output = format!("{:?}", sensor_error);
        assert!(debug_output.contains("SensorUnavailable"));
        assert!(debug_output.contains("GPU Clock"));
    }

    #[test]
    fn test_hardware_error_is_error_trait() {
        let error = HardwareError::ReadFailure("Test error".to_string());
        let _: &dyn std::error::Error = &error; // Should compile if Error trait is implemented
    }

    #[test]
    fn test_update_cpu_metrics_basic() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that CPU metrics update doesn't panic
        let result = poller.update_cpu_metrics();
        assert!(result.is_ok());
        
        // Verify that some CPU data was collected (utilization should always be available)
        let app_state = state.read();
        assert!(app_state.cpu.utilization.current.is_some());
    }

    #[test]
    fn test_update_gpu_metrics_basic() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that GPU metrics update doesn't panic
        let result = poller.update_gpu_metrics();
        assert!(result.is_ok());
        
        // GPU metrics may or may not be available depending on the system
        // Just verify the method executes without error
    }

    #[test]
    fn test_update_memory_metrics_basic() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that memory metrics update doesn't panic
        let result = poller.update_memory_metrics();
        assert!(result.is_ok());
        
        // Memory utilization should always be available
        let app_state = state.read();
        assert!(app_state.memory.utilization_mb.current.is_some());
    }

    #[test]
    fn test_update_storage_metrics_basic() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that storage metrics update doesn't panic
        let result = poller.update_storage_metrics();
        assert!(result.is_ok());
        
        // Storage metrics may not be available through sysinfo
        // Just verify the method executes without error
    }

    #[test]
    fn test_update_motherboard_metrics_basic() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that motherboard metrics update doesn't panic
        let result = poller.update_motherboard_metrics();
        assert!(result.is_ok());
        
        // Motherboard metrics may or may not be available depending on the system
        // Just verify the method executes without error
    }

    #[test]
    fn test_poll_hardware_comprehensive() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Capture initial state
        let initial_cpu_util = {
            let app_state = state.read();
            app_state.cpu.utilization.current
        };
        
        // Poll hardware
        poller.poll_hardware();
        
        // Verify that polling updated metrics
        let app_state = state.read();
        
        // CPU utilization should be available after polling
        assert!(app_state.cpu.utilization.current.is_some());
        
        // Memory utilization should be available after polling
        assert!(app_state.memory.utilization_mb.current.is_some());
        
        // Verify that utilization is within reasonable bounds
        if let Some(cpu_util) = app_state.cpu.utilization.current {
            assert!(cpu_util >= 0.0 && cpu_util <= 100.0);
        }
        
        if let Some(mem_util) = app_state.memory.utilization_mb.current {
            assert!(mem_util > 0); // Should have some memory usage
        }
    }

    #[test]
    fn test_get_cpu_temperature() {
        let state = create_test_state();
        let poller = HardwarePoller::new(state, 1000);
        
        // Test temperature retrieval (may or may not find CPU temperature)
        let temp = poller.get_cpu_temperature();
        
        // If temperature is found, it should be reasonable
        if let Some(temperature) = temp {
            assert!(temperature > -50.0 && temperature < 150.0); // Reasonable CPU temp range
        }
    }

    #[test]
    fn test_get_gpu_temperature() {
        let state = create_test_state();
        let poller = HardwarePoller::new(state, 1000);
        
        // Test GPU temperature retrieval (may or may not find GPU temperature)
        let temp = poller.get_gpu_temperature();
        
        // If temperature is found, it should be reasonable
        if let Some(temperature) = temp {
            assert!(temperature > -50.0 && temperature < 150.0); // Reasonable GPU temp range
        }
    }

    #[test]
    fn test_get_memory_temperature() {
        let state = create_test_state();
        let poller = HardwarePoller::new(state, 1000);
        
        // Test memory temperature retrieval (may or may not find memory temperature)
        let temp = poller.get_memory_temperature();
        
        // If temperature is found, it should be reasonable
        if let Some(temperature) = temp {
            assert!(temperature > -50.0 && temperature < 150.0); // Reasonable memory temp range
        }
    }

    #[test]
    fn test_multiple_polling_cycles() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Poll multiple times to test metric history
        poller.poll_hardware();
        poller.poll_hardware();
        poller.poll_hardware();
        
        let app_state = state.read();
        
        // Verify history is being maintained
        assert!(app_state.cpu.utilization.history.len() >= 3);
        
        // Verify min/max tracking works
        if app_state.cpu.utilization.current.is_some() {
            assert!(app_state.cpu.utilization.session_min.is_some());
            assert!(app_state.cpu.utilization.session_max.is_some());
        }
    }

    #[test]
    fn test_concurrent_polling_access() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Simulate concurrent access during polling
        std::thread::scope(|s| {
            // Start polling in a separate thread
            s.spawn(|| {
                poller.poll_hardware();
            });
            
            // Access state from main thread
            let app_state = state.read();
            // Just verify we can read without deadlock
            let _polling_interval = app_state.polling_interval_ms;
        });
    }

    #[test]
    fn test_metric_bounds_validation() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        poller.poll_hardware();
        
        let app_state = state.read();
        
        // Test CPU utilization bounds
        if let Some(cpu_util) = app_state.cpu.utilization.current {
            assert!(cpu_util >= 0.0 && cpu_util <= 100.0, 
                "CPU utilization should be between 0-100%, got: {}", cpu_util);
        }
        
        // Test memory utilization bounds (should be positive)
        if let Some(mem_util) = app_state.memory.utilization_mb.current {
            assert!(mem_util > 0, 
                "Memory utilization should be positive, got: {}", mem_util);
        }
        
        // Test frequency bounds (should be positive if available)
        if let Some(cpu_freq) = app_state.cpu.clock_speed.current {
            assert!(cpu_freq > 0, 
                "CPU frequency should be positive, got: {}", cpu_freq);
        }
        
        // Test temperature bounds (reasonable ranges if available)
        if let Some(cpu_temp) = app_state.cpu.package_temperature.current {
            assert!(cpu_temp > -50.0 && cpu_temp < 150.0,
                "CPU temperature should be reasonable, got: {}°C", cpu_temp);
        }
    }

    #[test]
    fn test_hardware_result_ok() {
        let result: HardwareResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_hardware_result_err() {
        let result: HardwareResult<i32> = Err(HardwareError::SensorUnavailable("test".to_string()));
        assert!(result.is_err());
        
        match result {
            Err(HardwareError::SensorUnavailable(msg)) => assert_eq!(msg, "test"),
            _ => panic!("Expected SensorUnavailable error"),
        }
    }

    #[test]
    fn test_polling_interval_consistency() {
        let intervals = [100, 500, 1000, 2000, 5000];
        
        for &interval_ms in &intervals {
            let state = create_test_state();
            let poller = HardwarePoller::new(state, interval_ms);
            assert_eq!(poller.polling_interval, Duration::from_millis(interval_ms));
        }
    }

    #[test]
    fn test_system_refresh_calls() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state, 1000);
        
        // Verify that polling works without system refresh errors
        // This indirectly tests that refresh_all() and components.refresh() work
        for _ in 0..5 {
            poller.poll_hardware();
        }
        
        // If we get here without panicking, the refresh calls work
        assert!(true);
    }

    #[test]
    fn test_error_handling_robustness() {
        let state = create_test_state();
        let mut poller = HardwarePoller::new(state.clone(), 1000);
        
        // Test that individual metric update failures don't crash the whole system
        let _cpu_result = poller.update_cpu_metrics();
        let _gpu_result = poller.update_gpu_metrics();
        let _memory_result = poller.update_memory_metrics();
        let _storage_result = poller.update_storage_metrics();
        let _motherboard_result = poller.update_motherboard_metrics();
        
        // The poller should still be functional
        poller.poll_hardware();
        
        // State should be accessible
        let app_state = state.read();
        assert_eq!(app_state.polling_interval_ms, 100);
    }

    // Mock test for thread safety (doesn't actually start threads due to test complexity)
    #[test]
    fn test_thread_safety_design() {
        let state = create_test_state();
        let poller = HardwarePoller::new(state.clone(), 1000);
        
        // Verify that the poller can be moved into a thread (would compile only if Send)
        let _handle = std::thread::spawn(move || {
            // Poller moved here, would fail compilation if not Send
            drop(poller);
        });
        
        // Verify state can be shared across threads
        let state_clone = state.clone();
        let _handle2 = std::thread::spawn(move || {
            let _app_state = state_clone.read();
        });
    }
}