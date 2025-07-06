use crate::hardware_detection::{HardwareMonitor, HardwareInfo};
use crate::model::SharedAppState;
use sysinfo::{System, Components};

pub struct GenericMonitor {
    system: System,
    components: Components,
    initialized: bool,
}

impl GenericMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            components: Components::new_with_refreshed_list(),
            initialized: false,
        }
    }
    
    fn get_cpu_temperature(&self) -> Option<f32> {
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("cpu") || label.contains("core") || label.contains("package") {
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

impl HardwareMonitor for GenericMonitor {
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.refresh_all();
        self.components.refresh();
        self.initialized = true;
        crate::logger::log_info("Generic monitor initialized using sysinfo");
        Ok(())
    }
    
    fn update_metrics(&mut self, state: &SharedAppState) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Ok(());
        }
        
        // Refresh system information
        self.system.refresh_all();
        self.components.refresh();
        
        let mut app_state = state.write();
        
        // CPU metrics that sysinfo can provide
        let cpu_usage = self.system.global_cpu_usage();
        app_state.cpu.utilization.update(cpu_usage);
        
        // CPU frequency from first core
        if let Some(cpu) = self.system.cpus().first() {
            let frequency_mhz = cpu.frequency() as u32;
            if frequency_mhz > 0 {
                app_state.cpu.clock_speed.update(frequency_mhz);
            }
        }
        
        // CPU temperature
        if let Some(temp) = self.get_cpu_temperature() {
            app_state.cpu.package_temperature.update(temp);
        }
        
        // Memory utilization
        let used_memory = self.system.used_memory();
        let usage_mb = (used_memory / 1024 / 1024) as u64;
        app_state.memory.utilization_mb.update(usage_mb);
        
        // Memory temperature
        if let Some(temp) = self.get_memory_temperature() {
            app_state.memory.temperature.update(temp);
        }
        
        // GPU temperature (basic fallback)
        if app_state.gpu.package_temperature.current.is_none() {
            if let Some(temp) = self.get_gpu_temperature() {
                app_state.gpu.package_temperature.update(temp);
            }
        }
        
        Ok(())
    }
    
    fn supports_hardware(&self, _info: &HardwareInfo) -> bool {
        // Generic monitor supports all hardware as a fallback
        true
    }
}