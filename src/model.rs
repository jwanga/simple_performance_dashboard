use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MetricValue<T> {
    pub current: Option<T>,
    pub session_min: Option<T>,
    pub session_max: Option<T>,
    pub history: VecDeque<(DateTime<Utc>, T)>,
}

impl<T> Default for MetricValue<T> 
where 
    T: Clone + PartialOrd,
{
    fn default() -> Self {
        Self {
            current: None,
            session_min: None,
            session_max: None,
            history: VecDeque::new(), // Full session history
        }
    }
}

impl<T> MetricValue<T> 
where 
    T: Clone + PartialOrd,
{
    pub fn update(&mut self, value: T) {
        let timestamp = Utc::now();
        
        // Update current value
        self.current = Some(value.clone());
        
        // Update session min/max
        if let Some(ref min) = self.session_min {
            if value < *min {
                self.session_min = Some(value.clone());
            }
        } else {
            self.session_min = Some(value.clone());
        }
        
        if let Some(ref max) = self.session_max {
            if value > *max {
                self.session_max = Some(value.clone());
            }
        } else {
            self.session_max = Some(value.clone());
        }
        
        // Add to history (keep full session history)
        self.history.push_back((timestamp, value));
    }
    
    pub fn get_history_for_plot(&self, session_start: DateTime<Utc>) -> Vec<(f64, f64)> {
        self.history
            .iter()
            .map(|(timestamp, value)| {
                let elapsed_seconds = (*timestamp - session_start).num_seconds() as f64;
                (elapsed_seconds, self.value_to_f64(value))
            })
            .collect()
    }
    
    fn value_to_f64(&self, _value: &T) -> f64 {
        // This is a placeholder - in practice, you'd implement this for each concrete type
        // For now, we'll handle this in the specific metric implementations
        0.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct CpuMetrics {
    pub utilization: MetricValue<f32>,           // Percentage
    pub clock_speed: MetricValue<u32>,           // MHz
    pub core_voltage: MetricValue<f32>,          // Volts
    pub power_consumption: MetricValue<f32>,     // Watts
    pub package_temperature: MetricValue<f32>,   // Celsius
    pub hotspot_temperature: MetricValue<f32>,   // Celsius
    pub thermal_throttling: MetricValue<bool>,   // Active/Inactive
}

#[derive(Debug, Clone, Default)]
pub struct GpuMetrics {
    pub utilization: MetricValue<f32>,           // Percentage
    pub clock_speed: MetricValue<u32>,           // MHz
    pub memory_utilization: MetricValue<u64>,    // MB
    pub core_voltage: MetricValue<f32>,          // Volts
    pub power_consumption: MetricValue<f32>,     // Watts
    pub package_temperature: MetricValue<f32>,   // Celsius
    pub hotspot_temperature: MetricValue<f32>,   // Celsius
    pub thermal_throttling: MetricValue<bool>,   // Active/Inactive
}

#[derive(Debug, Clone, Default)]
pub struct MemoryMetrics {
    pub utilization_mb: MetricValue<u64>,        // MB
    pub clock_speed: MetricValue<u32>,           // MHz
    pub temperature: MetricValue<f32>,           // Celsius
}

#[derive(Debug, Clone, Default)]
pub struct StorageMetrics {
    pub read_speed: MetricValue<f32>,              // MB/s
    pub write_speed: MetricValue<f32>,             // MB/s
    pub temperature: MetricValue<f32>,             // Celsius
}

#[derive(Debug, Clone, Default)]
pub struct MotherboardMetrics {
    pub chipset_temperature: MetricValue<f32>,     // Celsius
    pub chassis_temperature: MetricValue<f32>,     // Celsius
    pub aio_pump_speed: MetricValue<u32>,          // RPM
    pub chassis_fan_speed: MetricValue<u32>,       // RPM
    pub chipset_fan_speed: MetricValue<u32>,       // RPM
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub cpu: CpuMetrics,
    pub gpu: GpuMetrics,
    pub memory: MemoryMetrics,
    pub storage: StorageMetrics,
    pub motherboard: MotherboardMetrics,
    pub polling_interval_ms: u64,
    pub session_start: DateTime<Utc>,
    pub ui_state: UiState,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub cpu_section_expanded: bool,
    pub gpu_section_expanded: bool,
    pub memory_section_expanded: bool,
    pub storage_section_expanded: bool,
    pub motherboard_section_expanded: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            cpu_section_expanded: true,    // Default expanded
            gpu_section_expanded: true,    // Default expanded
            memory_section_expanded: true, // Default expanded
            storage_section_expanded: true, // Default expanded
            motherboard_section_expanded: true, // Default expanded
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cpu: CpuMetrics::default(),
            gpu: GpuMetrics::default(),
            memory: MemoryMetrics::default(),
            storage: StorageMetrics::default(),
            motherboard: MotherboardMetrics::default(),
            polling_interval_ms: 1000,
            session_start: Utc::now(),
            ui_state: UiState::default(),
        }
    }
}

pub type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn new(polling_interval_ms: u64) -> Self {
        Self {
            polling_interval_ms,
            session_start: Utc::now(),
            ..Default::default()
        }
    }
    
    pub fn new_shared(polling_interval_ms: u64) -> SharedAppState {
        Arc::new(RwLock::new(Self::new(polling_interval_ms)))
    }
    
    pub fn has_cpu_data(&self) -> bool {
        self.cpu.utilization.current.is_some() || 
        self.cpu.clock_speed.current.is_some() || 
        self.cpu.package_temperature.current.is_some()
    }
    
    pub fn has_gpu_data(&self) -> bool {
        self.gpu.clock_speed.current.is_some() || 
        self.gpu.package_temperature.current.is_some()
    }
    
    pub fn has_memory_data(&self) -> bool {
        self.memory.utilization_mb.current.is_some() ||
        self.memory.clock_speed.current.is_some() ||
        self.memory.temperature.current.is_some()
    }
    
    pub fn has_storage_data(&self) -> bool {
        self.storage.read_speed.current.is_some() ||
        self.storage.write_speed.current.is_some() ||
        self.storage.temperature.current.is_some()
    }
    
    pub fn has_motherboard_data(&self) -> bool {
        self.motherboard.chipset_temperature.current.is_some() || 
        self.motherboard.chassis_temperature.current.is_some() ||
        self.motherboard.aio_pump_speed.current.is_some() ||
        self.motherboard.chassis_fan_speed.current.is_some() ||
        self.motherboard.chipset_fan_speed.current.is_some()
    }
}

// Helper trait for converting values to f64 for plotting
pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for u64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for bool {
    fn to_f64(&self) -> f64 {
        if *self { 1.0 } else { 0.0 }
    }
}

impl<T: ToF64> MetricValue<T> {
    pub fn get_plot_data(&self, session_start: DateTime<Utc>) -> Vec<(f64, f64)> {
        self.history
            .iter()
            .map(|(timestamp, value)| {
                let elapsed_seconds = (*timestamp - session_start).num_seconds() as f64;
                (elapsed_seconds, value.to_f64())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_metric_value_default() {
        let metric: MetricValue<f32> = MetricValue::default();
        assert!(metric.current.is_none());
        assert!(metric.session_min.is_none());
        assert!(metric.session_max.is_none());
        assert!(metric.history.is_empty());
    }

    #[test]
    fn test_metric_value_single_update() {
        let mut metric = MetricValue::default();
        metric.update(50.0f32);

        assert_eq!(metric.current, Some(50.0));
        assert_eq!(metric.session_min, Some(50.0));
        assert_eq!(metric.session_max, Some(50.0));
        assert_eq!(metric.history.len(), 1);
    }

    #[test]
    fn test_metric_value_multiple_updates() {
        let mut metric = MetricValue::default();
        
        metric.update(50.0f32);
        metric.update(30.0f32);
        metric.update(70.0f32);
        metric.update(40.0f32);

        assert_eq!(metric.current, Some(40.0));
        assert_eq!(metric.session_min, Some(30.0));
        assert_eq!(metric.session_max, Some(70.0));
        assert_eq!(metric.history.len(), 4);
    }

    #[test]
    fn test_metric_value_min_max_tracking() {
        let mut metric = MetricValue::default();
        
        // Test ascending values
        metric.update(10.0f32);
        assert_eq!(metric.session_min, Some(10.0));
        assert_eq!(metric.session_max, Some(10.0));
        
        metric.update(20.0f32);
        assert_eq!(metric.session_min, Some(10.0));
        assert_eq!(metric.session_max, Some(20.0));
        
        // Test value between min and max
        metric.update(15.0f32);
        assert_eq!(metric.session_min, Some(10.0));
        assert_eq!(metric.session_max, Some(20.0));
        
        // Test new minimum
        metric.update(5.0f32);
        assert_eq!(metric.session_min, Some(5.0));
        assert_eq!(metric.session_max, Some(20.0));
        
        // Test new maximum
        metric.update(25.0f32);
        assert_eq!(metric.session_min, Some(5.0));
        assert_eq!(metric.session_max, Some(25.0));
    }

    #[test]
    fn test_metric_value_boolean() {
        let mut metric = MetricValue::default();
        
        metric.update(false);
        assert_eq!(metric.current, Some(false));
        assert_eq!(metric.session_min, Some(false));
        assert_eq!(metric.session_max, Some(false));
        
        metric.update(true);
        assert_eq!(metric.current, Some(true));
        assert_eq!(metric.session_min, Some(false));
        assert_eq!(metric.session_max, Some(true));
        
        metric.update(false);
        assert_eq!(metric.current, Some(false));
        assert_eq!(metric.session_min, Some(false));
        assert_eq!(metric.session_max, Some(true));
    }

    #[test]
    fn test_metric_value_history_ordering() {
        let mut metric = MetricValue::default();
        
        let start_time = Utc::now();
        metric.update(10.0f32);
        
        // Small delay to ensure different timestamps
        thread::sleep(StdDuration::from_millis(1));
        metric.update(20.0f32);
        
        thread::sleep(StdDuration::from_millis(1));
        metric.update(30.0f32);
        
        assert_eq!(metric.history.len(), 3);
        
        // Verify chronological order
        let timestamps: Vec<DateTime<Utc>> = metric.history.iter().map(|(t, _)| *t).collect();
        for i in 1..timestamps.len() {
            assert!(timestamps[i] >= timestamps[i-1]);
        }
        
        // Verify values are in correct order
        let values: Vec<f32> = metric.history.iter().map(|(_, v)| *v).collect();
        assert_eq!(values, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_to_f64_trait() {
        assert_eq!((42.5f32).to_f64(), 42.5);
        assert_eq!((100u32).to_f64(), 100.0);
        assert_eq!((1000u64).to_f64(), 1000.0);
        assert_eq!(true.to_f64(), 1.0);
        assert_eq!(false.to_f64(), 0.0);
    }

    #[test]
    fn test_metric_value_plot_data() {
        let mut metric = MetricValue::default();
        let session_start = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        
        // Add data points with known timestamps
        let timestamp1 = session_start + Duration::seconds(10);
        let timestamp2 = session_start + Duration::seconds(20);
        let timestamp3 = session_start + Duration::seconds(30);
        
        metric.history.push_back((timestamp1, 50.0f32));
        metric.history.push_back((timestamp2, 75.0f32));
        metric.history.push_back((timestamp3, 25.0f32));
        
        let plot_data = metric.get_plot_data(session_start);
        
        assert_eq!(plot_data.len(), 3);
        assert_eq!(plot_data[0], (10.0, 50.0));
        assert_eq!(plot_data[1], (20.0, 75.0));
        assert_eq!(plot_data[2], (30.0, 25.0));
    }

    #[test]
    fn test_cpu_metrics_default() {
        let cpu = CpuMetrics::default();
        assert!(cpu.utilization.current.is_none());
        assert!(cpu.clock_speed.current.is_none());
        assert!(cpu.core_voltage.current.is_none());
        assert!(cpu.power_consumption.current.is_none());
        assert!(cpu.package_temperature.current.is_none());
        assert!(cpu.hotspot_temperature.current.is_none());
        assert!(cpu.thermal_throttling.current.is_none());
    }

    #[test]
    fn test_gpu_metrics_default() {
        let gpu = GpuMetrics::default();
        assert!(gpu.utilization.current.is_none());
        assert!(gpu.clock_speed.current.is_none());
        assert!(gpu.memory_utilization.current.is_none());
        assert!(gpu.core_voltage.current.is_none());
        assert!(gpu.power_consumption.current.is_none());
        assert!(gpu.package_temperature.current.is_none());
        assert!(gpu.hotspot_temperature.current.is_none());
        assert!(gpu.thermal_throttling.current.is_none());
    }

    #[test]
    fn test_memory_metrics_default() {
        let memory = MemoryMetrics::default();
        assert!(memory.utilization_mb.current.is_none());
        assert!(memory.clock_speed.current.is_none());
        assert!(memory.temperature.current.is_none());
    }

    #[test]
    fn test_storage_metrics_default() {
        let storage = StorageMetrics::default();
        assert!(storage.read_speed.current.is_none());
        assert!(storage.write_speed.current.is_none());
        assert!(storage.temperature.current.is_none());
    }

    #[test]
    fn test_motherboard_metrics_default() {
        let motherboard = MotherboardMetrics::default();
        assert!(motherboard.chipset_temperature.current.is_none());
        assert!(motherboard.chassis_temperature.current.is_none());
        assert!(motherboard.aio_pump_speed.current.is_none());
        assert!(motherboard.chassis_fan_speed.current.is_none());
        assert!(motherboard.chipset_fan_speed.current.is_none());
    }

    #[test]
    fn test_ui_state_default() {
        let ui_state = UiState::default();
        assert!(ui_state.cpu_section_expanded);
        assert!(ui_state.gpu_section_expanded);
        assert!(ui_state.memory_section_expanded);
        assert!(ui_state.storage_section_expanded);
        assert!(ui_state.motherboard_section_expanded);
    }

    #[test]
    fn test_app_state_default() {
        let app_state = AppState::default();
        assert_eq!(app_state.polling_interval_ms, 1000);
        assert!(app_state.ui_state.cpu_section_expanded);
        assert!(!app_state.has_cpu_data());
        assert!(!app_state.has_gpu_data());
        assert!(!app_state.has_memory_data());
        assert!(!app_state.has_storage_data());
        assert!(!app_state.has_motherboard_data());
    }

    #[test]
    fn test_app_state_new() {
        let app_state = AppState::new(500);
        assert_eq!(app_state.polling_interval_ms, 500);
        assert!(!app_state.has_cpu_data());
    }

    #[test]
    fn test_app_state_new_shared() {
        let shared_state = AppState::new_shared(2000);
        let state = shared_state.read();
        assert_eq!(state.polling_interval_ms, 2000);
    }

    #[test]
    fn test_has_cpu_data() {
        let mut app_state = AppState::default();
        assert!(!app_state.has_cpu_data());
        
        app_state.cpu.utilization.update(50.0);
        assert!(app_state.has_cpu_data());
        
        let mut app_state2 = AppState::default();
        app_state2.cpu.clock_speed.update(3000);
        assert!(app_state2.has_cpu_data());
        
        let mut app_state3 = AppState::default();
        app_state3.cpu.package_temperature.update(65.0);
        assert!(app_state3.has_cpu_data());
    }

    #[test]
    fn test_has_gpu_data() {
        let mut app_state = AppState::default();
        assert!(!app_state.has_gpu_data());
        
        app_state.gpu.clock_speed.update(1500);
        assert!(app_state.has_gpu_data());
        
        let mut app_state2 = AppState::default();
        app_state2.gpu.package_temperature.update(70.0);
        assert!(app_state2.has_gpu_data());
    }

    #[test]
    fn test_has_memory_data() {
        let mut app_state = AppState::default();
        assert!(!app_state.has_memory_data());
        
        app_state.memory.utilization_mb.update(8192);
        assert!(app_state.has_memory_data());
        
        let mut app_state2 = AppState::default();
        app_state2.memory.clock_speed.update(3200);
        assert!(app_state2.has_memory_data());
        
        let mut app_state3 = AppState::default();
        app_state3.memory.temperature.update(45.0);
        assert!(app_state3.has_memory_data());
    }

    #[test]
    fn test_has_storage_data() {
        let mut app_state = AppState::default();
        assert!(!app_state.has_storage_data());
        
        app_state.storage.read_speed.update(500.0);
        assert!(app_state.has_storage_data());
        
        let mut app_state2 = AppState::default();
        app_state2.storage.write_speed.update(300.0);
        assert!(app_state2.has_storage_data());
        
        let mut app_state3 = AppState::default();
        app_state3.storage.temperature.update(40.0);
        assert!(app_state3.has_storage_data());
    }

    #[test]
    fn test_has_motherboard_data() {
        let mut app_state = AppState::default();
        assert!(!app_state.has_motherboard_data());
        
        app_state.motherboard.chipset_temperature.update(55.0);
        assert!(app_state.has_motherboard_data());
        
        let mut app_state2 = AppState::default();
        app_state2.motherboard.chassis_temperature.update(35.0);
        assert!(app_state2.has_motherboard_data());
        
        let mut app_state3 = AppState::default();
        app_state3.motherboard.aio_pump_speed.update(2500);
        assert!(app_state3.has_motherboard_data());
        
        let mut app_state4 = AppState::default();
        app_state4.motherboard.chassis_fan_speed.update(1200);
        assert!(app_state4.has_motherboard_data());
        
        let mut app_state5 = AppState::default();
        app_state5.motherboard.chipset_fan_speed.update(800);
        assert!(app_state5.has_motherboard_data());
    }

    #[test]
    fn test_shared_app_state_concurrent_access() {
        let shared_state = AppState::new_shared(1000);
        
        // Test concurrent reads
        let state1 = shared_state.read();
        let state2 = shared_state.read();
        assert_eq!(state1.polling_interval_ms, state2.polling_interval_ms);
        drop(state1);
        drop(state2);
        
        // Test write access
        {
            let mut state = shared_state.write();
            state.cpu.utilization.update(75.0);
        }
        
        // Verify write was successful
        let state = shared_state.read();
        assert_eq!(state.cpu.utilization.current, Some(75.0));
    }

    #[test]
    fn test_metric_edge_cases() {
        let mut metric = MetricValue::default();
        
        // Test with zero values
        metric.update(0.0f32);
        assert_eq!(metric.current, Some(0.0));
        assert_eq!(metric.session_min, Some(0.0));
        assert_eq!(metric.session_max, Some(0.0));
        
        // Test with negative values
        metric.update(-10.0f32);
        assert_eq!(metric.current, Some(-10.0));
        assert_eq!(metric.session_min, Some(-10.0));
        assert_eq!(metric.session_max, Some(0.0));
        
        // Test with very large values
        metric.update(f32::MAX);
        assert_eq!(metric.current, Some(f32::MAX));
        assert_eq!(metric.session_min, Some(-10.0));
        assert_eq!(metric.session_max, Some(f32::MAX));
    }

    #[test]
    fn test_session_start_timing() {
        let before = Utc::now();
        let app_state = AppState::new(1000);
        let after = Utc::now();
        
        assert!(app_state.session_start >= before);
        assert!(app_state.session_start <= after);
    }
}