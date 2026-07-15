use sysmon_engine::MetricsCollector;
use std::thread;
use std::time::Duration;

fn fmt_opt_f32(v: Option<f32>) -> String {
    match v {
        Some(x) => format!("{:.0}", x),
        None => "N/A".to_string(),
    }
}

fn fmt_opt_u32(v: Option<u32>) -> String {
    match v {
        Some(x) => format!("{}", x),
        None => "N/A".to_string(),
    }
}

fn main() {
    println!("Iniciando monitor... (Ctrl+C para salir)\n");
    let mut collector = MetricsCollector::new();

    loop {
        thread::sleep(Duration::from_secs(3));
        let m = collector.collect();
        println!(
            "CPU: {:>5.1}% | CPUT: {:>3}°C | RAM: {:>5.1}% | AMD-GPU: {:>3}% | AMD-T: {:>3}°C | NV-GPU: {:>3}% | NV-T: {:>3}°C",
            m.cpu_usage,
            fmt_opt_f32(m.cpu_temp),
            m.ram_usage,
            fmt_opt_f32(m.amd_gpu_usage),
            fmt_opt_f32(m.amd_gpu_temp),
            fmt_opt_u32(m.nvidia_gpu_usage),
            fmt_opt_u32(m.nvidia_gpu_temp),
        );
    }
}