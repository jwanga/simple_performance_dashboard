use simple_performance_dashboard::model::AppState;
use simple_performance_dashboard::hardware::HardwarePoller;
use simple_performance_dashboard::ui::interpolate_data_value;
use chrono::Utc;

#[test]
fn test_end_to_end_data_flow() {
    // Create shared application state
    let state = AppState::new_shared(100); // 100ms polling for faster testing
    
    // Create hardware poller
    let mut poller = HardwarePoller::new(state.clone(), 100);
    
    // Poll hardware multiple times to simulate real usage
    for _ in 0..3 {
        poller.poll_hardware();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    // Verify that data was collected and state was updated
    let app_state = state.read();
    
    // CPU utilization should be available
    assert!(app_state.cpu.utilization.current.is_some());
    
    // Memory utilization should be available
    assert!(app_state.memory.utilization_mb.current.is_some());
    
    // History should contain multiple entries
    assert!(app_state.cpu.utilization.history.len() >= 3);
    
    // Min/max should be tracked
    if app_state.cpu.utilization.current.is_some() {
        assert!(app_state.cpu.utilization.session_min.is_some());
        assert!(app_state.cpu.utilization.session_max.is_some());
    }
}

#[test]
fn test_concurrent_polling_and_ui_access() {
    let state = AppState::new_shared(50); // Fast polling for testing
    let mut poller = HardwarePoller::new(state.clone(), 50);
    
    // Simulate concurrent access - polling in background while UI reads data
    std::thread::scope(|s| {
        // Background polling thread
        let state_clone = state.clone();
        s.spawn(move || {
            for _ in 0..10 {
                poller.poll_hardware();
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });
        
        // UI thread reading data
        s.spawn(move || {
            for _ in 0..20 {
                let app_state = state_clone.read();
                // Simulate UI operations
                let _has_cpu_data = app_state.has_cpu_data();
                let _has_memory_data = app_state.has_memory_data();
                let _polling_interval = app_state.polling_interval_ms;
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        });
    });
    
    // Verify final state is consistent
    let final_state = state.read();
    assert!(final_state.cpu.utilization.history.len() > 0);
}

#[test]
fn test_data_availability_detection() {
    let mut state = AppState::default();
    
    // Initially no data should be available
    assert!(!state.has_cpu_data());
    assert!(!state.has_gpu_data());
    assert!(!state.has_memory_data());
    assert!(!state.has_storage_data());
    assert!(!state.has_motherboard_data());
    
    // Add CPU data
    state.cpu.utilization.update(50.0);
    assert!(state.has_cpu_data());
    
    // Add GPU data
    state.gpu.clock_speed.update(1500);
    assert!(state.has_gpu_data());
    
    // Add memory data
    state.memory.utilization_mb.update(8192);
    assert!(state.has_memory_data());
    
    // Add storage data
    state.storage.read_speed.update(500.0);
    assert!(state.has_storage_data());
    
    // Add motherboard data
    state.motherboard.chipset_temperature.update(55.0);
    assert!(state.has_motherboard_data());
}

#[test]
fn test_metric_history_and_plotting() {
    let session_start = Utc::now();
    let state = AppState::new_shared(100);
    
    // Add data points with specific timestamps
    {
        let mut app_state = state.write();
        app_state.cpu.utilization.update(25.0);
        std::thread::sleep(std::time::Duration::from_millis(50));
        app_state.cpu.utilization.update(50.0);
        std::thread::sleep(std::time::Duration::from_millis(50));
        app_state.cpu.utilization.update(75.0);
    }
    
    // Test plot data generation
    let app_state = state.read();
    let plot_data = app_state.cpu.utilization.get_plot_data(session_start);
    
    assert_eq!(plot_data.len(), 3);
    
    // Verify values are correct
    assert_eq!(plot_data[0].1, 25.0);
    assert_eq!(plot_data[1].1, 50.0);
    assert_eq!(plot_data[2].1, 75.0);
    
    // Verify time progression (allow for some timestamp resolution issues)
    if plot_data.len() >= 2 {
        assert!(plot_data[0].0 <= plot_data[1].0);
        if plot_data.len() >= 3 {
            assert!(plot_data[1].0 <= plot_data[2].0);
        }
    }
}

#[test]
fn test_interpolate_data_value_function() {
    // Test data interpolation for crosshair functionality
    let data = vec![
        (10.0, 50.0),
        (20.0, 100.0),
        (30.0, 75.0),
    ];
    
    // Test exact matches
    assert_eq!(interpolate_data_value(&data, 10.0), Some(50.0));
    assert_eq!(interpolate_data_value(&data, 20.0), Some(100.0));
    assert_eq!(interpolate_data_value(&data, 30.0), Some(75.0));
    
    // Test interpolation between points
    let interpolated = interpolate_data_value(&data, 15.0).unwrap();
    assert!((interpolated - 75.0).abs() < 0.1); // Should be midway between 50 and 100
    
    // Test extrapolation
    assert_eq!(interpolate_data_value(&data, 5.0), Some(50.0)); // Before first point
    assert_eq!(interpolate_data_value(&data, 35.0), Some(75.0)); // After last point
    
    // Test empty data
    assert_eq!(interpolate_data_value(&[], 15.0), None);
}

#[test]
fn test_session_timing_consistency() {
    let before_creation = Utc::now();
    let state = AppState::new(1000);
    let after_creation = Utc::now();
    
    // Session start should be between creation times
    assert!(state.session_start >= before_creation);
    assert!(state.session_start <= after_creation);
    
    // Test shared state creation timing
    let before_shared = Utc::now();
    let shared_state = AppState::new_shared(500);
    let after_shared = Utc::now();
    
    let shared_app_state = shared_state.read();
    assert!(shared_app_state.session_start >= before_shared);
    assert!(shared_app_state.session_start <= after_shared);
}

#[test]
fn test_metric_value_bounds_and_types() {
    let mut state = AppState::default();
    
    // Test different metric types and bounds
    
    // Float metrics (percentages, temperatures, voltages)
    state.cpu.utilization.update(87.5);
    state.cpu.core_voltage.update(1.35);
    state.cpu.package_temperature.update(65.2);
    
    // Integer metrics (frequencies, speeds, memory)
    state.cpu.clock_speed.update(3400);
    state.memory.utilization_mb.update(16384);
    state.motherboard.aio_pump_speed.update(2500);
    
    // Boolean metrics (throttling status)
    state.cpu.thermal_throttling.update(false);
    state.gpu.thermal_throttling.update(true);
    
    // Verify all updates were successful
    assert_eq!(state.cpu.utilization.current, Some(87.5));
    assert_eq!(state.cpu.core_voltage.current, Some(1.35));
    assert_eq!(state.cpu.package_temperature.current, Some(65.2));
    assert_eq!(state.cpu.clock_speed.current, Some(3400));
    assert_eq!(state.memory.utilization_mb.current, Some(16384));
    assert_eq!(state.motherboard.aio_pump_speed.current, Some(2500));
    assert_eq!(state.cpu.thermal_throttling.current, Some(false));
    assert_eq!(state.gpu.thermal_throttling.current, Some(true));
}

#[test]
fn test_ui_state_consistency() {
    let state = AppState::default();
    
    // Verify default UI state
    assert!(state.ui_state.cpu_section_expanded);
    assert!(state.ui_state.gpu_section_expanded);
    assert!(state.ui_state.memory_section_expanded);
    assert!(state.ui_state.storage_section_expanded);
    assert!(state.ui_state.motherboard_section_expanded);
    
    // Test shared state UI consistency
    let shared_state = AppState::new_shared(1000);
    let shared_app_state = shared_state.read();
    assert!(shared_app_state.ui_state.cpu_section_expanded);
    assert!(shared_app_state.ui_state.gpu_section_expanded);
    assert!(shared_app_state.ui_state.memory_section_expanded);
    assert!(shared_app_state.ui_state.storage_section_expanded);
    assert!(shared_app_state.ui_state.motherboard_section_expanded);
}

#[test]
fn test_polling_interval_configuration() {
    let intervals = [100, 500, 1000, 2000, 5000];
    
    for &interval in &intervals {
        let state = AppState::new(interval);
        assert_eq!(state.polling_interval_ms, interval);
        
        let shared_state = AppState::new_shared(interval);
        let shared_app_state = shared_state.read();
        assert_eq!(shared_app_state.polling_interval_ms, interval);
    }
}

#[test]
fn test_metric_min_max_tracking_comprehensive() {
    let mut state = AppState::default();
    
    // Test CPU utilization min/max tracking
    let cpu_values = [45.0, 80.0, 30.0, 95.0, 60.0];
    for &value in &cpu_values {
        state.cpu.utilization.update(value);
    }
    
    assert_eq!(state.cpu.utilization.session_min, Some(30.0));
    assert_eq!(state.cpu.utilization.session_max, Some(95.0));
    assert_eq!(state.cpu.utilization.current, Some(60.0));
    
    // Test memory utilization min/max tracking
    let memory_values = [4096, 8192, 2048, 12288, 6144];
    for &value in &memory_values {
        state.memory.utilization_mb.update(value);
    }
    
    assert_eq!(state.memory.utilization_mb.session_min, Some(2048));
    assert_eq!(state.memory.utilization_mb.session_max, Some(12288));
    assert_eq!(state.memory.utilization_mb.current, Some(6144));
    
    // Test boolean min/max tracking (thermal throttling)
    state.cpu.thermal_throttling.update(false);
    state.cpu.thermal_throttling.update(true);
    state.cpu.thermal_throttling.update(false);
    
    assert_eq!(state.cpu.thermal_throttling.session_min, Some(false));
    assert_eq!(state.cpu.thermal_throttling.session_max, Some(true));
    assert_eq!(state.cpu.thermal_throttling.current, Some(false));
}

#[test]
fn test_application_state_lifecycle() {
    // Test complete application lifecycle
    
    // 1. Application startup
    let state = AppState::new_shared(1000);
    assert_eq!(state.read().polling_interval_ms, 1000);
    
    // 2. Hardware polling initialization
    let mut poller = HardwarePoller::new(state.clone(), 1000);
    
    // 3. Initial polling cycle
    poller.poll_hardware();
    
    // 4. Verify initial data collection
    {
        let app_state = state.read();
        // At least CPU and memory should have data
        assert!(app_state.cpu.utilization.current.is_some());
        assert!(app_state.memory.utilization_mb.current.is_some());
    }
    
    // 5. Multiple polling cycles (simulating runtime)
    for _ in 0..5 {
        poller.poll_hardware();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    // 6. Verify continuous data collection
    {
        let app_state = state.read();
        assert!(app_state.cpu.utilization.history.len() >= 6);
        assert!(app_state.memory.utilization_mb.history.len() >= 6);
    }
    
    // 7. Verify UI can access all required data
    {
        let app_state = state.read();
        
        // Test data availability detection
        let has_cpu = app_state.has_cpu_data();
        let has_memory = app_state.has_memory_data();
        let has_gpu = app_state.has_gpu_data();
        let has_storage = app_state.has_storage_data();
        let has_motherboard = app_state.has_motherboard_data();
        
        // CPU and memory should definitely have data
        assert!(has_cpu);
        assert!(has_memory);
        
        // Others may or may not have data depending on system
        // Just verify the detection doesn't crash
        let _all_sections = [has_cpu, has_memory, has_gpu, has_storage, has_motherboard];
    }
}

#[test]
fn test_data_consistency_under_load() {
    let state = AppState::new_shared(10); // Very fast polling
    
    // Simulate high-frequency updates
    std::thread::scope(|s| {
        // Multiple updater threads
        for thread_id in 0..3 {
            let state_clone = state.clone();
            s.spawn(move || {
                for i in 0..100 {
                    let value = (thread_id * 100 + i) as f32;
                    let mut app_state = state_clone.write();
                    app_state.cpu.utilization.update(value % 100.0); // Keep within valid range
                }
            });
        }
        
        // Reader threads (simulating UI)
        for _ in 0..2 {
            let state_clone = state.clone();
            s.spawn(move || {
                for _ in 0..200 {
                    let app_state = state_clone.read();
                    let _current = app_state.cpu.utilization.current;
                    let _min = app_state.cpu.utilization.session_min;
                    let _max = app_state.cpu.utilization.session_max;
                    let _history_len = app_state.cpu.utilization.history.len();
                }
            });
        }
    });
    
    // Verify final state is consistent
    let final_state = state.read();
    
    // Should have collected lots of data
    assert!(final_state.cpu.utilization.history.len() >= 100);
    
    // Min/max should be within expected bounds
    if let (Some(min), Some(max)) = (
        final_state.cpu.utilization.session_min,
        final_state.cpu.utilization.session_max
    ) {
        assert!(min <= max);
        assert!(min >= 0.0 && min <= 100.0);
        assert!(max >= 0.0 && max <= 100.0);
    }
    
    // Current value should be valid
    if let Some(current) = final_state.cpu.utilization.current {
        assert!(current >= 0.0 && current <= 100.0);
    }
}