use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;
use sensors::Sensors;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub cpu_temp: Option<f32>,
    pub ram_usage: f32,
    pub amd_gpu_usage: Option<f32>,
    pub amd_gpu_temp: Option<f32>,
    pub nvidia_gpu_usage: Option<u32>,
    pub nvidia_gpu_temp: Option<u32>,
}

pub struct MetricsCollector {
    amd_card_path: Option<PathBuf>,
    nvml: Option<Nvml>,
    prev_cpu_times: (u64, u64),
}

impl MetricsCollector {
    pub fn new() -> Self {
        let amd_path = Self::find_amd_gpu_path();
        let nvml = Nvml::init().ok();

        if amd_path.is_none() {
            println!("[aviso] No se detectó GPU AMD por sysfs.");
        }
        if nvml.is_none() {
            println!("[aviso] No se pudo inicializar NVML (¿driver Nvidia activo?).");
        }

        let prev_cpu_times = Self::read_cpu_times().unwrap_or((0, 0));

        Self {
            amd_card_path: amd_path,
            nvml,
            prev_cpu_times,
        }
    }

    fn find_amd_gpu_path() -> Option<PathBuf> {
        let entries = fs::read_dir("/sys/class/drm").ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_string();
            if name.starts_with("card") && !name.contains('-') {
                let vendor_path = path.join("device/vendor");
                if let Ok(vendor_hex) = fs::read_to_string(&vendor_path) {
                    if vendor_hex.trim() == "0x1002" {
                        return Some(path.join("device"));
                    }
                }
            }
        }
        None
    }

    fn find_amd_hwmon_temp_path(amd_device_path: &PathBuf) -> Option<PathBuf> {
        let hwmon_dir = amd_device_path.join("hwmon");
        let entries = fs::read_dir(hwmon_dir).ok()?;
        for entry in entries.flatten() {
            let temp_path = entry.path().join("temp1_input");
            if temp_path.exists() {
                return Some(temp_path);
            }
        }
        None
    }

    fn read_cpu_times() -> Option<(u64, u64)> {
        let content = fs::read_to_string("/proc/stat").ok()?;
        let first_line = content.lines().next()?;
        let values: Vec<u64> = first_line
            .split_whitespace()
            .skip(1)
            .filter_map(|v| v.parse::<u64>().ok())
            .collect();

        if values.len() < 4 {
            return None;
        }

        let idle = values[3];
        let total: u64 = values.iter().sum();
        Some((idle, total))
    }

    fn cpu_usage_percent(prev: (u64, u64), curr: (u64, u64)) -> f32 {
        let idle_delta = curr.0.saturating_sub(prev.0) as f32;
        let total_delta = curr.1.saturating_sub(prev.1) as f32;

        if total_delta == 0.0 {
            return 0.0;
        }

        100.0 * (1.0 - idle_delta / total_delta)
    }

    fn read_ram_usage_percent() -> Option<f32> {
        let content = fs::read_to_string("/proc/meminfo").ok()?;
        let mut total_kb = 0u64;
        let mut available_kb = 0u64;

        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            } else if line.starts_with("MemAvailable:") {
                available_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            }
        }

        if total_kb == 0 {
            return None;
        }

        Some(100.0 * (1.0 - available_kb as f32 / total_kb as f32))
    }

    fn read_cpu_temp() -> Option<f32> {
        let sensors = Sensors::new();
        for chip in sensors {
            for feature in chip {
                let label = feature.get_label().unwrap_or_default();
                if label.contains("Package") || label.contains("Tctl") || label.contains("Tdie") {
                    for sub in feature {
                        if let Ok(value) = sub.get_value() {
                            return Some(value as f32);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn collect(&mut self) -> SystemMetrics {
        let mut metrics = SystemMetrics::default();

        let curr_cpu = Self::read_cpu_times().unwrap_or(self.prev_cpu_times);
        metrics.cpu_usage = Self::cpu_usage_percent(self.prev_cpu_times, curr_cpu);
        self.prev_cpu_times = curr_cpu;

        metrics.ram_usage = Self::read_ram_usage_percent().unwrap_or(0.0);

        metrics.cpu_temp = Self::read_cpu_temp();

        if let Some(ref path) = self.amd_card_path {
            if let Ok(busy) = fs::read_to_string(path.join("gpu_busy_percent")) {
                metrics.amd_gpu_usage = busy.trim().parse::<f32>().ok();
            }
            if let Some(temp_path) = Self::find_amd_hwmon_temp_path(path) {
                if let Ok(temp_raw) = fs::read_to_string(temp_path) {
                    if let Ok(temp_milli) = temp_raw.trim().parse::<f32>() {
                        metrics.amd_gpu_temp = Some(temp_milli / 1000.0);
                    }
                }
            }
        }

        if let Some(ref nvml) = self.nvml {
            if let Ok(device) = nvml.device_by_index(0) {
                if let Ok(utilization) = device.utilization_rates() {
                    metrics.nvidia_gpu_usage = Some(utilization.gpu);
                }
                if let Ok(temp) = device.temperature(TemperatureSensor::Gpu) {
                    metrics.nvidia_gpu_temp = Some(temp);
                }
            }
        }

        metrics
    }
}