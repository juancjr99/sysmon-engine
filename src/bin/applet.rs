use cosmic::widget::text;
use cosmic::iced::time;
use cosmic::iced::alignment::Vertical;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use sysmon_engine::{MetricsCollector, SystemMetrics};

const APP_ID: &str = "com.juancjr.SysMon";
const CPU_ICON_SVG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/icons/cpu.svg"));
const GPU_ICON_SVG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/icons/gpu.svg"));
const RAM_ICON_SVG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/icons/memory-stick.svg"));

struct SysMonApplet {
    core: cosmic::Core,
    collector: Arc<Mutex<MetricsCollector>>,
    metrics: SystemMetrics,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    MetricsLoaded(SystemMetrics),
}

impl cosmic::Application for SysMonApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let collector = Arc::new(Mutex::new(MetricsCollector::new()));
        let applet = Self {
            core,
            collector,
            metrics: SystemMetrics::default(),
        };
        (applet, cosmic::app::Task::none())
    }

    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::Tick => {
                let collector = self.collector.clone();
                cosmic::iced::Task::perform(
                    async move {
                        let mut guard = collector.lock().unwrap();
                        guard.collect()
                    },
                    Message::MetricsLoaded,
                )
                .map(Into::into)
            }
            Message::MetricsLoaded(new_metrics) => {
                if std::env::var_os("SYSMON_DEBUG").is_some() {
                    eprintln!(
                        "[sysmon] metrics cpu={:.1}% ram={:.1}% amd={:?}/{:?} nv={:?}/{:?}",
                        new_metrics.cpu_usage,
                        new_metrics.ram_usage,
                        new_metrics.amd_gpu_usage,
                        new_metrics.amd_gpu_temp,
                        new_metrics.nvidia_gpu_usage,
                        new_metrics.nvidia_gpu_temp,
                    );
                }
                self.metrics = new_metrics;
                cosmic::app::Task::none()
            }
        }
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        time::every(Duration::from_secs(3)).map(|_| Message::Tick)
    }

    fn view(&self) -> cosmic::Element<'_, Self::Message> {
        let amd_gpu = self.metrics.amd_gpu_usage
            .map(|u| format!("{:.0}%", u))
            .unwrap_or_else(|| "N/A".into());

        let nv_gpu = self.metrics.nvidia_gpu_usage
            .map(|u| format!("{}%", u))
            .unwrap_or_else(|| "N/A".into());

        let gpu_display = if self.metrics.amd_gpu_usage.is_some() {
            amd_gpu
        } else {
            nv_gpu
        };

        let cpu_display = format!("{:.0}%", self.metrics.cpu_usage);
        let ram_display = format!("{:.0}%", self.metrics.ram_usage);

        let cpu_icon = cosmic::widget::icon::from_svg_bytes(CPU_ICON_SVG)
            .symbolic(true)
            .icon()
            .size(13);
        let gpu_icon = cosmic::widget::icon::from_svg_bytes(GPU_ICON_SVG)
            .symbolic(true)
            .icon()
            .size(13);
        let ram_icon = cosmic::widget::icon::from_svg_bytes(RAM_ICON_SVG)
            .symbolic(true)
            .icon()
            .size(13);

        let content = cosmic::iced::widget::row![
            cpu_icon,
            text(cpu_display).size(11),
            gpu_icon,
            text(gpu_display).size(11),
            ram_icon,
            text(ram_display).size(11),
        ]
        .spacing(4)
        .align_y(Vertical::Center);

        self.core.applet.autosize_window(content).into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<SysMonApplet>(())
}
