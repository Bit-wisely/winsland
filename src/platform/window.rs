//! The single island window: borderless, topmost, no taskbar button,
//! Composition owns all pixels (no redirection bitmap).
use std::error::Error;
use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::ActiveEventLoop,
    platform::windows::WindowAttributesExtWindows,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Window, WindowLevel},
};

use super::display::MonitorRect;
use crate::core::island::{WINDOW_H, WINDOW_W};

pub struct IslandWindow {
    pub window: Window,
    pub hwnd: HWND,
    pub scale: f32,
}

pub fn create(el: &ActiveEventLoop) -> Result<IslandWindow, Box<dyn Error>> {
    let mon = el
        .primary_monitor()
        .or_else(|| el.available_monitors().next())
        .map(|m| MonitorRect {
            x: m.position().x,
            y: m.position().y,
            w: m.size().width,
            h: m.size().height,
            scale: m.scale_factor() as f32,
        })
        .unwrap_or(MonitorRect::FALLBACK);

    let w = (WINDOW_W * mon.scale) as i32;
    let h = (WINDOW_H * mon.scale) as u32;
    let (x, y) = mon.top_centre(w);

    let attrs = Window::default_attributes()
        .with_title("WinIsland")
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_skip_taskbar(true)
        .with_undecorated_shadow(false)
        .with_no_redirection_bitmap(true)
        .with_inner_size(PhysicalSize::new(w as u32, h))
        .with_position(PhysicalPosition::new(x, y));

    let window = el.create_window(attrs)?;
    let hwnd = match window.window_handle()?.as_raw() {
        RawWindowHandle::Win32(h) => HWND(h.hwnd.get() as *mut c_void),
        _ => return Err("not a Win32 window".into()),
    };
    Ok(IslandWindow { window, hwnd, scale: mon.scale })
}
