//! Monitor geometry helpers (physical pixels).

#[derive(Debug, Clone, Copy)]
pub struct MonitorRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub scale: f32,
}

impl MonitorRect {
    pub const FALLBACK: MonitorRect = MonitorRect { x: 0, y: 0, w: 1920, h: 1080, scale: 1.0 };

    /// Top-left position that centres a window of `win_w` physical px on this monitor's top edge.
    pub fn top_centre(&self, win_w: i32) -> (i32, i32) {
        (self.x + (self.w as i32 - win_w) / 2, self.y)
    }
}

// TODO(phase 2): react to monitor hotplug / DPI change (WindowEvent::ScaleFactorChanged).
