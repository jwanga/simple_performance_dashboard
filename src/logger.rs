use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Utc;
use log::{error, warn, info};

pub struct AppLogger {
    log_file_path: PathBuf,
}

impl AppLogger {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create log file in application directory
        let mut log_path = std::env::current_exe()?;
        log_path.pop(); // Remove executable name
        log_path.push("simple_performance_dashboard.log");
        
        let logger = Self {
            log_file_path: log_path,
        };
        
        // Create/rewrite the log file for this session
        logger.initialize_log_file()?;
        
        Ok(logger)
    }
    
    fn initialize_log_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.log_file_path)?;
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "{} [INFO] === Simple Performance Dashboard Session Started ===", timestamp)?;
        Ok(())
    }
    
    fn write_log_entry(&self, level: &str, message: &str) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.log_file_path) {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
            let _ = writeln!(file, "{} [{}] {}", timestamp, level, message);
        }
    }
    
    pub fn log_info(&self, message: &str) {
        self.write_log_entry("INFO", message);
        info!("{}", message);
    }
    
    pub fn log_warning(&self, message: &str) {
        self.write_log_entry("WARN", message);
        warn!("{}", message);
    }
    
    pub fn log_error(&self, context: &str, error: &dyn std::error::Error) {
        let message = format!("{}: {}", context, error);
        self.write_log_entry("ERROR", &message);
        error!("{}", message);
    }
    
    pub fn log_sensor_error(&self, sensor_name: &str, error: &dyn std::error::Error) {
        let message = format!("Sensor error - {}: {}", sensor_name, error);
        self.write_log_entry("ERROR", &message);
        error!("{}", message);
    }
    
    pub fn log_sensor_unavailable(&self, sensor_name: &str) {
        let message = format!("Sensor unavailable: {}", sensor_name);
        self.write_log_entry("WARN", &message);
        warn!("{}", message);
    }
    
    pub fn log_hardware_polling_error(&self, error: &dyn std::error::Error) {
        let message = format!("Hardware polling error: {}", error);
        self.write_log_entry("ERROR", &message);
        error!("{}", message);
    }
}

// Global logger instance
static mut LOGGER: Option<AppLogger> = None;

pub fn initialize_logger() -> Result<(), Box<dyn std::error::Error>> {
    let logger = AppLogger::new()?;
    unsafe {
        LOGGER = Some(logger);
    }
    Ok(())
}

pub fn log_info(message: &str) {
    unsafe {
        if let Some(ref logger) = LOGGER {
            logger.log_info(message);
        }
    }
}

pub fn log_warning(message: &str) {
    unsafe {
        if let Some(ref logger) = LOGGER {
            logger.log_warning(message);
        }
    }
}

pub fn log_error(context: &str, error: &dyn std::error::Error) {
    unsafe {
        if let Some(ref logger) = LOGGER {
            logger.log_error(context, error);
        }
    }
}

pub fn log_sensor_error(sensor_name: &str, error: &dyn std::error::Error) {
    unsafe {
        if let Some(ref logger) = LOGGER {
            logger.log_sensor_error(sensor_name, error);
        }
    }
}

pub fn log_sensor_unavailable(sensor_name: &str) {
    unsafe {
        if let Some(ref logger) = LOGGER {
            logger.log_sensor_unavailable(sensor_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_logger_creation() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        // Verify logger was created successfully
        assert!(!logger.log_file_path.to_string_lossy().is_empty());
    }

    #[test]
    fn test_log_info() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        // Test that log_info doesn't panic
        logger.log_info("Test info message");
    }

    #[test]
    fn test_log_warning() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        // Test that log_warning doesn't panic
        logger.log_warning("Test warning message");
    }

    #[test]
    fn test_log_error() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        let test_error = std::io::Error::new(std::io::ErrorKind::NotFound, "Test error");
        
        // Test that log_error doesn't panic
        logger.log_error("Test error context", &test_error);
    }

    #[test]
    fn test_log_sensor_error() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        let test_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        
        // Test that log_sensor_error doesn't panic
        logger.log_sensor_error("CPU Temperature", &test_error);
    }

    #[test]
    fn test_log_sensor_unavailable() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        // Test that log_sensor_unavailable doesn't panic
        logger.log_sensor_unavailable("GPU Clock Speed");
    }

    #[test]
    fn test_log_hardware_polling_error() {
        let logger = AppLogger::new().expect("Failed to create logger");
        
        let test_error = std::io::Error::new(std::io::ErrorKind::TimedOut, "Polling timeout");
        
        // Test that log_hardware_polling_error doesn't panic
        logger.log_hardware_polling_error(&test_error);
    }

    #[test]
    fn test_global_logger_initialization() {
        let result = initialize_logger();
        assert!(result.is_ok());
    }

    #[test]
    fn test_global_logger_functions() {
        // Initialize logger first
        let _ = initialize_logger();
        
        // Test global logging functions don't panic
        log_info("Global info test");
        log_warning("Global warning test");
        
        let test_error = std::io::Error::new(std::io::ErrorKind::Other, "Global error test");
        log_error("Global error context", &test_error);
        
        log_sensor_error("Test Sensor", &test_error);
        log_sensor_unavailable("Test Unavailable Sensor");
    }

    #[test]
    fn test_global_logger_without_initialization() {
        // Reset global logger
        unsafe {
            LOGGER = None;
        }
        
        // These should not panic even without initialization
        log_info("Should not crash");
        log_warning("Should not crash");
        
        let test_error = std::io::Error::new(std::io::ErrorKind::Other, "Should not crash");
        log_error("Should not crash", &test_error);
        log_sensor_error("Should not crash", &test_error);
        log_sensor_unavailable("Should not crash");
    }
}