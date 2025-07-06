use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Corner, CoordinatesFormatter};
use egui::CollapsingHeader;
use crate::model::{SharedAppState, MetricValue, ToF64};

// Helper function to interpolate data value at a given time position
pub fn interpolate_data_value(data: &[(f64, f64)], target_time: f64) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    
    // Find the closest data points around the target time
    let mut before_idx = None;
    let mut after_idx = None;
    
    for (i, &(time, _)) in data.iter().enumerate() {
        if time <= target_time {
            before_idx = Some(i);
        }
        if time >= target_time && after_idx.is_none() {
            after_idx = Some(i);
            break;
        }
    }
    
    match (before_idx, after_idx) {
        (Some(before), Some(after)) if before == after => {
            // Exact match
            Some(data[before].1)
        }
        (Some(before), Some(after)) => {
            // Interpolate between two points
            let (t1, v1) = data[before];
            let (t2, v2) = data[after];
            
            if t2 == t1 {
                Some(v1)
            } else {
                let ratio = (target_time - t1) / (t2 - t1);
                Some(v1 + ratio * (v2 - v1))
            }
        }
        (Some(before), None) => {
            // Use the last available data point
            Some(data[before].1)
        }
        (None, Some(after)) => {
            // Use the first available data point
            Some(data[after].1)
        }
        _ => None,
    }
}

pub struct PerformanceApp {
    state: SharedAppState,
}

impl PerformanceApp {
    pub fn new(state: SharedAppState) -> Self {
        Self { state }
    }
    
    fn render_metric_section<T>(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        metric: &MetricValue<T>,
        unit: &str,
        format_fn: impl Fn(&T) -> String,
        session_start: chrono::DateTime<chrono::Utc>,
    ) where
        T: ToF64 + Clone,
    {
        ui.group(|ui| {
            ui.label(egui::RichText::new(title).heading());
            
            ui.horizontal(|ui| {
                // Current value
                if let Some(ref current) = metric.current {
                    ui.label(format!("Current: {}{}", format_fn(current), unit));
                } else {
                    ui.label("Current: N/A");
                }
                
                ui.separator();
                
                // Session min/max
                if let (Some(ref min), Some(ref max)) = (&metric.session_min, &metric.session_max) {
                    ui.label(format!("Min: {}{}", format_fn(min), unit));
                    ui.label(format!("Max: {}{}", format_fn(max), unit));
                } else {
                    ui.label("Min: N/A");
                    ui.label("Max: N/A");
                }
            });
            
            // Plot - always show, even if no data
            let plot_data = metric.get_plot_data(session_start);
            let elapsed_seconds = (chrono::Utc::now() - session_start).num_seconds() as f64;
            
            // Calculate Y-axis bounds from session min/max values
            let (y_min, y_max) = if let (Some(ref min), Some(ref max)) = (&metric.session_min, &metric.session_max) {
                let min_val = min.to_f64();
                let max_val = max.to_f64();
                // Add 5% padding to bounds for better visualization
                let padding = (max_val - min_val) * 0.05;
                (min_val - padding, max_val + padding)
            } else {
                // Default bounds when no data available
                (0.0, 100.0)
            };
            
            Plot::new(format!("{}_plot", title))
                .height(100.0)
                .label_formatter(|_name, _value| String::new())
                .coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::new({
                    let plot_data_clone = plot_data.clone();
                    move |point, _bounds| {
                        if point.x >= 0.0 && !plot_data_clone.is_empty() {
                            // Find the actual data value at the cursor time position
                            let cursor_time = point.x;
                            let interpolated_value = interpolate_data_value(&plot_data_clone, cursor_time);
                            
                            if let Some(value) = interpolated_value {
                                format!("Time: {:.1}s, {}: {:.1}{}", 
                                    cursor_time, 
                                    title,
                                    value, 
                                    unit
                                )
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    }
                }))
                .show(ui, |plot_ui| {
                    if !plot_data.is_empty() {
                        let points: PlotPoints = plot_data.into_iter().map(|(x, y)| [x, y]).collect();
                        let line = Line::new(points);
                        plot_ui.line(line);
                    }
                    // Set bounds: X-axis from 0 to elapsed time, Y-axis to session min/max
                    plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                        [0.0, y_min], 
                        [elapsed_seconds.max(1.0), y_max]
                    ));
                });
        });
    }
    
    fn render_cpu_section(&self, ui: &mut egui::Ui) {
        let state = self.state.read();
        let session_start = state.session_start;
        let has_data = state.has_cpu_data();
        
        // Determine if section should be open based on requirements:
        // - Sections with data: default expanded
        // - Sections without data: default collapsed  
        let should_be_open = has_data;
        
        let section_title = if has_data { "CPU Metrics" } else { "CPU Metrics (No Data)" };
        let text_color = if has_data { egui::Color32::WHITE } else { egui::Color32::GRAY };
        
        CollapsingHeader::new(egui::RichText::new(section_title).color(text_color))
            .default_open(should_be_open)
            .show(ui, |ui| {
            ui.columns(2, |columns| {
                // Left column
                self.render_metric_section(
                    &mut columns[0],
                    "CPU Utilization",
                    &state.cpu.utilization,
                    "%",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "CPU Clock Speed",
                    &state.cpu.clock_speed,
                    " MHz",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "CPU Core Voltage",
                    &state.cpu.core_voltage,
                    " V",
                    |v| format!("{:.2}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "CPU Power Consumption",
                    &state.cpu.power_consumption,
                    " W",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column
                self.render_metric_section(
                    &mut columns[1],
                    "CPU Package Temperature",
                    &state.cpu.package_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[1],
                    "CPU Hotspot Temperature",
                    &state.cpu.hotspot_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column continued - Thermal throttling as a proper metric
                self.render_metric_section(
                    &mut columns[1],
                    "CPU Thermal Throttling",
                    &state.cpu.thermal_throttling,
                    "",
                    |v| if *v { "1=Active".to_string() } else { "0=Inactive".to_string() },
                    session_start,
                );
            });
        });
    }
    
    fn render_gpu_section(&self, ui: &mut egui::Ui) {
        let state = self.state.read();
        let session_start = state.session_start;
        let has_data = state.has_gpu_data();
        
        // Determine if section should be open based on requirements:
        // - Sections with data: default expanded
        // - Sections without data: default collapsed  
        let should_be_open = has_data;
        
        let section_title = if has_data { "GPU Metrics" } else { "GPU Metrics (No Data)" };
        let text_color = if has_data { egui::Color32::WHITE } else { egui::Color32::GRAY };
        
        CollapsingHeader::new(egui::RichText::new(section_title).color(text_color))
            .default_open(should_be_open)
            .show(ui, |ui| {
            ui.columns(2, |columns| {
                // Left column
                self.render_metric_section(
                    &mut columns[0],
                    "GPU Utilization",
                    &state.gpu.utilization,
                    "%",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "GPU Clock Speed",
                    &state.gpu.clock_speed,
                    " MHz",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "GPU Memory Utilization",
                    &state.gpu.memory_utilization,
                    " MB",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "GPU Core Voltage",
                    &state.gpu.core_voltage,
                    " V",
                    |v| format!("{:.2}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "GPU Power Consumption",
                    &state.gpu.power_consumption,
                    " W",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column
                self.render_metric_section(
                    &mut columns[1],
                    "GPU Package Temperature",
                    &state.gpu.package_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[1],
                    "GPU Hotspot Temperature",
                    &state.gpu.hotspot_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column continued - Thermal throttling as a proper metric
                self.render_metric_section(
                    &mut columns[1],
                    "GPU Thermal Throttling",
                    &state.gpu.thermal_throttling,
                    "",
                    |v| if *v { "1=Active".to_string() } else { "0=Inactive".to_string() },
                    session_start,
                );
            });
        });
    }
    
    fn render_memory_section(&self, ui: &mut egui::Ui) {
        let state = self.state.read();
        let session_start = state.session_start;
        let has_data = state.has_memory_data();
        
        // Determine if section should be open based on requirements:
        // - Sections with data: default expanded
        // - Sections without data: default collapsed  
        let should_be_open = has_data;
        
        let section_title = if has_data { "Memory Metrics" } else { "Memory Metrics (No Data)" };
        let text_color = if has_data { egui::Color32::WHITE } else { egui::Color32::GRAY };
        
        CollapsingHeader::new(egui::RichText::new(section_title).color(text_color))
            .default_open(should_be_open)
            .show(ui, |ui| {
            ui.columns(2, |columns| {
                // Left column
                self.render_metric_section(
                    &mut columns[0],
                    "Memory Utilization",
                    &state.memory.utilization_mb,
                    " MB",
                    |v| format!("{}", v),
                    session_start,
                );
                
                // Right column
                self.render_metric_section(
                    &mut columns[1],
                    "Memory Clock Speed",
                    &state.memory.clock_speed,
                    " MHz",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[1],
                    "Memory Temperature",
                    &state.memory.temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
            });
        });
    }
    
    fn render_storage_section(&self, ui: &mut egui::Ui) {
        let state = self.state.read();
        let session_start = state.session_start;
        let has_data = state.has_storage_data();
        
        // Determine if section should be open based on requirements:
        // - Sections with data: default expanded
        // - Sections without data: default collapsed  
        let should_be_open = has_data;
        
        let section_title = if has_data { "Storage Metrics" } else { "Storage Metrics (No Data)" };
        let text_color = if has_data { egui::Color32::WHITE } else { egui::Color32::GRAY };
        
        CollapsingHeader::new(egui::RichText::new(section_title).color(text_color))
            .default_open(should_be_open)
            .show(ui, |ui| {
            ui.columns(2, |columns| {
                // Left column
                self.render_metric_section(
                    &mut columns[0],
                    "Drive Read Speed",
                    &state.storage.read_speed,
                    " MB/s",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "Drive Write Speed",
                    &state.storage.write_speed,
                    " MB/s",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column
                self.render_metric_section(
                    &mut columns[1],
                    "Drive Temperature",
                    &state.storage.temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
            });
        });
    }
    
    fn render_motherboard_section(&self, ui: &mut egui::Ui) {
        let state = self.state.read();
        let session_start = state.session_start;
        let has_data = state.has_motherboard_data();
        
        // Determine if section should be open based on requirements:
        // - Sections with data: default expanded
        // - Sections without data: default collapsed  
        let should_be_open = has_data;
        
        let section_title = if has_data { "Motherboard Metrics" } else { "Motherboard Metrics (No Data)" };
        let text_color = if has_data { egui::Color32::WHITE } else { egui::Color32::GRAY };
        
        CollapsingHeader::new(egui::RichText::new(section_title).color(text_color))
            .default_open(should_be_open)
            .show(ui, |ui| {
            ui.columns(2, |columns| {
                // Left column - Temperatures
                self.render_metric_section(
                    &mut columns[0],
                    "Chipset Temperature",
                    &state.motherboard.chipset_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[0],
                    "Chassis Temperature",
                    &state.motherboard.chassis_temperature,
                    "°C",
                    |v| format!("{:.1}", v),
                    session_start,
                );
                
                // Right column - Fan Speeds
                self.render_metric_section(
                    &mut columns[1],
                    "AIO Pump Speed",
                    &state.motherboard.aio_pump_speed,
                    " RPM",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[1],
                    "Chassis Fan Speed",
                    &state.motherboard.chassis_fan_speed,
                    " RPM",
                    |v| format!("{}", v),
                    session_start,
                );
                
                self.render_metric_section(
                    &mut columns[1],
                    "Chipset Fan Speed",
                    &state.motherboard.chipset_fan_speed,
                    " RPM",
                    |v| format!("{}", v),
                    session_start,
                );
            });
        });
    }
}

impl eframe::App for PerformanceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint for continuous updates
        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Performance Dashboard");
            
            ui.separator();
            
            // Display polling interval
            {
                let state = self.state.read();
                ui.horizontal(|ui| {
                    ui.label("Polling Interval:");
                    ui.label(format!("{} ms", state.polling_interval_ms));
                });
            }
            
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_cpu_section(ui);
                ui.separator();
                
                self.render_gpu_section(ui);
                ui.separator();
                
                self.render_memory_section(ui);
                ui.separator();
                
                self.render_storage_section(ui);
                ui.separator();
                
                self.render_motherboard_section(ui);
            });
        });
    }
}

pub fn run_app(state: SharedAppState) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Simple Performance Dashboard"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Simple Performance Dashboard",
        options,
        Box::new(|_cc| Ok(Box::new(PerformanceApp::new(state)))),
    )
}