use super::paths;
use crate::ui::theme::ThemeKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: ThemeKind,
    pub run_at_startup: bool,
    pub hide_in_fullscreen: bool,
    /// Skip the backdrop brush (tint only). Useful when debugging blur.
    pub force_solid_backdrop: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeKind::default(),
            run_at_startup: false,
            hide_in_fullscreen: true,
            force_solid_backdrop: false,
        }
    }
}

impl Settings {
    /// Missing or invalid file -> defaults.
    pub fn load() -> Settings {
        std::fs::read_to_string(paths::settings_file())
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(paths::config_dir())?;
        let text = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(paths::settings_file(), text)
    }
}
