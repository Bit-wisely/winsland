//! Detect a fullscreen app / game / presentation so the island can hide.
//! TODO(phase 2): call this from a slow timer or foreground-window hook and
//! emit Event::FullscreenChanged when the result changes.
use windows::Win32::UI::Shell::{
    SHQueryUserNotificationState, QUNS_BUSY, QUNS_PRESENTATION_MODE, QUNS_RUNNING_D3D_FULL_SCREEN,
};

pub fn fullscreen_app_active() -> bool {
    unsafe {
        match SHQueryUserNotificationState() {
            Ok(s) => s == QUNS_BUSY || s == QUNS_RUNNING_D3D_FULL_SCREEN || s == QUNS_PRESENTATION_MODE,
            Err(_) => false,
        }
    }
}
