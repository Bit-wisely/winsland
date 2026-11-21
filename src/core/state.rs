use super::event::{BatteryInfo, Connectivity, MediaInfo, Notification, Stats};
use super::island::IslandMode;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub mode: IslandMode,
    pub hidden: bool,
    pub media: Option<MediaInfo>,
    pub battery: Option<BatteryInfo>,
    pub notification: Option<Notification>,
    pub focus_running: bool,
    pub volume: f32,
    pub brightness: f32,
    pub stats: Stats,
    pub connectivity: Connectivity,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: IslandMode::Idle,
            hidden: false,
            media: None,
            battery: None,
            notification: None,
            focus_running: false,
            volume: 0.0,
            brightness: 0.0,
            stats: Stats::default(),
            connectivity: Connectivity::default(),
        }
    }
}

impl AppState {
    /// The compact mode the island rests in when not expanded.
    /// Priority: notification > charging > media > focus > idle.
    /// TODO(phase 6): make Charging transient (a few seconds after plug-in).
    pub fn resting_mode(&self) -> IslandMode {
        if self.notification.is_some() {
            IslandMode::Notify
        } else if self.battery.as_ref().is_some_and(|b| b.charging) {
            IslandMode::Charging
        } else if self.media.as_ref().is_some_and(|m| m.playing) {
            IslandMode::Media
        } else if self.focus_running {
            IslandMode::Focus
        } else {
            IslandMode::Idle
        }
    }
}
