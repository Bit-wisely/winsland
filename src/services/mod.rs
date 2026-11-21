//! Services push `Event`s into the app's event loop. They never touch UI or
//! window code, and they wake the loop only when something actually changed.
use crate::config::settings::Settings;
use crate::core::event::Event;
use winit::event_loop::EventLoopProxy;

pub mod battery;
pub mod brightness;
pub mod calendar;
pub mod connectivity;
pub mod hardware_stats;
pub mod media;
pub mod notifications;
pub mod sensors;
pub mod volume;
pub mod weather;

/// Handle used by services to send events to the main thread.
pub type Tx = EventLoopProxy<Event>;
pub type ServiceResult = Result<(), Box<dyn std::error::Error>>;

pub trait Service {
    fn name(&self) -> &'static str;
    fn start(&mut self, tx: Tx) -> ServiceResult;
    fn stop(&mut self) {}
}

/// Start every enabled service; a failing service is logged and skipped.
pub fn start_all(tx: &Tx, _settings: &Settings) -> Vec<Box<dyn Service>> {
    let mut all: Vec<Box<dyn Service>> = vec![
        Box::new(media::MediaService::default()),
        Box::new(battery::BatteryService::default()),
        Box::new(volume::VolumeService::default()),
        Box::new(brightness::BrightnessService::default()),
        Box::new(notifications::NotificationService::default()),
        Box::new(connectivity::ConnectivityService::default()),
        Box::new(calendar::CalendarService::default()),
        Box::new(hardware_stats::HardwareStatsService::default()),
        // Optional, off by default (settings flags later):
        // Box::new(weather::WeatherService::default()),
        // Box::new(sensors::SensorsService::default()),
    ];
    all.retain_mut(|s| match s.start(tx.clone()) {
        Ok(()) => true,
        Err(e) => {
            tracing::error!("service {} failed to start: {e}", s.name());
            false
        }
    });
    all
}
