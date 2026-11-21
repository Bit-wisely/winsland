//! Everything that can happen. Services and the window emit these;
//! only the reducer interprets them.

#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub playing: bool,
    pub position_s: u32,
    pub duration_s: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BatteryInfo {
    pub percent: u8,
    pub charging: bool,
    pub minutes_left: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub app: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Stats {
    pub cpu: f32,
    pub gpu: f32,
    pub ram: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Connectivity {
    pub wifi: bool,
    pub bluetooth: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    // Input (from the window)
    Click,
    Hover(bool),
    Escape,
    Quit,
    // Services
    MediaChanged(Option<MediaInfo>),
    BatteryChanged(BatteryInfo),
    VolumeChanged(f32),     // 0.0..=1.0
    BrightnessChanged(f32), // 0.0..=1.0
    NotificationPosted(Notification),
    NotificationDismissed,
    ConnectivityChanged(Connectivity),
    StatsUpdated(Stats),
    FocusRunning(bool),
    // System
    FullscreenChanged(bool),
}
