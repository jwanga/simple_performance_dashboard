use crate::hardware_detection::{HardwareMonitor, HardwareInfo, GpuVendor};
use crate::model::SharedAppState;

#[cfg(feature = "nvidia")]
use nvml_wrapper::Nvml;

pub struct NvidiaMonitor {
    #[cfg(feature = "nvidia")]
    nvml: Option<Nvml>,
    initialized: bool,
}

impl NvidiaMonitor {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "nvidia")]
            nvml: None,
            initialized: false,
        }
    }
}

impl HardwareMonitor for NvidiaMonitor {
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "nvidia")]
        {
            match Nvml::init() {
                Ok(nvml) => {
                    self.nvml = Some(nvml);
                    self.initialized = true;
                    crate::logger::log_info("NVIDIA NVML initialized successfully");
                    Ok(())
                }
                Err(e) => {
                    crate::logger::log_error("Failed to initialize NVIDIA NVML", &e);
                    Err(Box::new(e))
                }
            }
        }
        
        #[cfg(not(feature = "nvidia"))]
        {
            Err("NVIDIA support not compiled in".into())
        }
    }
    
    fn update_metrics(&mut self, state: &SharedAppState) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "nvidia")]
        {
            if !self.initialized {
                return Ok(());
            }
            
            if let Some(ref nvml) = self.nvml {
                let mut app_state = state.write();
                
                // Try to get the first GPU device
                if let Ok(device_count) = nvml.device_count() {
                    if device_count > 0 {
                        if let Ok(device) = nvml.device_by_index(0) {
                            // GPU Utilization
                            if let Ok(utilization) = device.utilization_rates() {
                                app_state.gpu.utilization.update(utilization.gpu as f32);
                            }
                            
                            // GPU Clock Speed
                            if let Ok(clock_speed) = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics) {
                                app_state.gpu.clock_speed.update(clock_speed as u32);
                            }
                            
                            // GPU Memory Utilization
                            if let Ok(memory_info) = device.memory_info() {
                                let used_mb = (memory_info.used / 1024 / 1024) as u64;
                                app_state.gpu.memory_utilization.update(used_mb);
                            }
                            
                            // GPU Temperature
                            if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                                app_state.gpu.package_temperature.update(temp as f32);
                            }
                            
                            // GPU Power Consumption
                            if let Ok(power) = device.power_usage() {
                                let power_watts = (power as f32) / 1000.0; // Convert mW to W
                                app_state.gpu.power_consumption.update(power_watts);
                            }
                            
                            // GPU Thermal Throttling
                            if let Ok(throttle_reasons) = device.current_throttle_reasons() {
                                let is_throttling = !throttle_reasons.is_empty();
                                app_state.gpu.thermal_throttling.update(is_throttling);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn supports_hardware(&self, info: &HardwareInfo) -> bool {
        info.gpu_vendors.contains(&GpuVendor::NVIDIA)
    }
}