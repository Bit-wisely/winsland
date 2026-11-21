//! WinIsland library root.
//!
//! `core`, `config`, `render` (data types), `ui` and `services` compile on any
//! OS so the state machine can be unit-tested anywhere. Everything that talks
//! to Win32/WinRT is gated behind `cfg(windows)`.

pub mod config;
pub mod core;
pub mod render;
pub mod services;
pub mod ui;

#[cfg(windows)]
pub mod platform;

#[cfg(windows)]
pub mod app;
