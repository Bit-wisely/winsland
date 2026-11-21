use std::path::PathBuf;

/// `%APPDATA%\WinIsland`
pub fn config_dir() -> PathBuf {
    let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    base.join("WinIsland")
}

pub fn settings_file() -> PathBuf {
    config_dir().join("settings.toml")
}
