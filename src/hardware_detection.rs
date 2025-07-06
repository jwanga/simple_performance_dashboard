
#[derive(Debug, Clone, PartialEq)]
pub enum CpuVendor {
    Intel,
    AMD,
    Apple,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GpuVendor {
    NVIDIA,
    AMD,
    Intel,
    Apple,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct HardwareInfo {
    pub cpu_vendor: CpuVendor,
    pub gpu_vendors: Vec<GpuVendor>,
    pub platform: Platform,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

pub struct HardwareDetector;

impl HardwareDetector {
    pub fn detect() -> HardwareInfo {
        let platform = Self::detect_platform();
        let cpu_vendor = Self::detect_cpu_vendor();
        let gpu_vendors = Self::detect_gpu_vendors();
        
        HardwareInfo {
            cpu_vendor,
            gpu_vendors,
            platform,
        }
    }
    
    fn detect_platform() -> Platform {
        #[cfg(target_os = "windows")]
        return Platform::Windows;
        
        #[cfg(target_os = "macos")]
        return Platform::MacOS;
        
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return Platform::Unknown;
    }
    
    fn detect_cpu_vendor() -> CpuVendor {
        // Apple Silicon detection
        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        {
            return CpuVendor::Apple;
        }
        
        // For x86/x86_64, use CPUID to detect vendor
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if let Some(vendor) = Self::get_cpu_vendor_string() {
                match vendor.as_str() {
                    "GenuineIntel" => return CpuVendor::Intel,
                    "AuthenticAMD" => return CpuVendor::AMD,
                    _ => return CpuVendor::Unknown,
                }
            }
        }
        
        #[cfg(not(any(all(target_arch = "aarch64", target_os = "macos"), any(target_arch = "x86", target_arch = "x86_64"))))]
        {
            CpuVendor::Unknown
        }
        
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            CpuVendor::Unknown
        }
    }
    
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn get_cpu_vendor_string() -> Option<String> {
        // Use raw_cpuid crate for CPUID instruction
        #[cfg(feature = "cpuid")]
        {
            use raw_cpuid::CpuId;
            let cpuid = CpuId::new();
            if let Some(vendor_info) = cpuid.get_vendor_info() {
                return Some(vendor_info.as_str().to_string());
            }
        }
        None
    }
    
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn get_cpu_vendor_string() -> Option<String> {
        None
    }
    
    fn detect_gpu_vendors() -> Vec<GpuVendor> {
        let mut vendors = Vec::new();
        
        // Platform-specific GPU detection
        #[cfg(target_os = "windows")]
        {
            vendors.extend(Self::detect_windows_gpus());
        }
        
        #[cfg(target_os = "macos")]
        {
            vendors.extend(Self::detect_macos_gpus());
        }
        
        #[cfg(target_os = "linux")]
        {
            vendors.extend(Self::detect_linux_gpus());
        }
        
        // Fallback: try to detect through available APIs
        if vendors.is_empty() {
            vendors.extend(Self::detect_gpus_by_api());
        }
        
        vendors
    }
    
    #[cfg(target_os = "windows")]
    fn detect_windows_gpus() -> Vec<GpuVendor> {
        let mut vendors = Vec::new();
        
        // Try WMI query for GPU information
        // This is a simplified implementation
        // In production, you'd use proper WMI bindings
        
        vendors
    }
    
    #[cfg(target_os = "macos")]
    fn detect_macos_gpus() -> Vec<GpuVendor> {
        let mut vendors = Vec::new();
        
        // On Apple Silicon, there's always an Apple GPU
        #[cfg(target_arch = "aarch64")]
        {
            vendors.push(GpuVendor::Apple);
        }
        
        // Could also have discrete AMD/NVIDIA GPUs
        // Use Metal or IOKit to detect
        
        vendors
    }
    
    #[cfg(target_os = "linux")]
    fn detect_linux_gpus() -> Vec<GpuVendor> {
        let mut vendors = Vec::new();
        
        // Check /sys/class/drm for GPU devices
        // Check lspci output
        // This is a simplified implementation
        
        vendors
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    fn detect_windows_gpus() -> Vec<GpuVendor> { Vec::new() }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    fn detect_macos_gpus() -> Vec<GpuVendor> { Vec::new() }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    fn detect_linux_gpus() -> Vec<GpuVendor> { Vec::new() }
    
    fn detect_gpus_by_api() -> Vec<GpuVendor> {
        let mut vendors = Vec::new();
        
        // Try NVIDIA NVML
        #[cfg(feature = "nvidia")]
        {
            if let Ok(_nvml) = nvml_wrapper::Nvml::init() {
                vendors.push(GpuVendor::NVIDIA);
            }
        }
        
        // Try AMD ADL (when implemented)
        #[cfg(feature = "amd")]
        {
            // AMD GPU detection would go here
        }
        
        // Try Intel GPU APIs (when implemented)
        #[cfg(feature = "intel")]
        {
            // Intel GPU detection would go here
        }
        
        vendors
    }
}

// Trait for hardware-specific monitoring implementations
pub trait HardwareMonitor: Send + Sync {
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn update_metrics(&mut self, state: &crate::model::SharedAppState) -> Result<(), Box<dyn std::error::Error>>;
    fn supports_hardware(&self, info: &HardwareInfo) -> bool;
}