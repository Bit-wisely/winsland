//! Style tokens. Variants differ only by data, never by code path.
use crate::render::cmd::Rgba;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeKind {
    #[default]
    Fluent,
    Light,
    Acrylic,
    Mica,
    Glass,
    Compact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backdrop {
    /// CreateHostBackdropBrush (acrylic-like)
    HostAcrylic,
    /// No blur, tint only
    Solid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub kind: ThemeKind,
    pub backdrop: Backdrop,
    pub tint: Rgba,
    pub card: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub accent: Rgba,
    /// Compact variant: never expands the idle pill's footprint.
    pub minimal: bool,
}

impl Theme {
    pub fn for_kind(kind: ThemeKind) -> Theme {
        let dark = Theme {
            kind,
            backdrop: Backdrop::HostAcrylic,
            tint: Rgba::new(10, 14, 24, 90),
            card: Rgba::new(255, 255, 255, 18),
            text_primary: Rgba::new(255, 255, 255, 255),
            text_secondary: Rgba::new(255, 255, 255, 160),
            accent: Rgba::new(76, 160, 255, 255),
            minimal: false,
        };
        match kind {
            ThemeKind::Fluent => dark,
            ThemeKind::Light => Theme {
                tint: Rgba::new(240, 244, 250, 120),
                card: Rgba::new(0, 0, 0, 14),
                text_primary: Rgba::new(20, 24, 32, 255),
                text_secondary: Rgba::new(20, 24, 32, 150),
                ..dark
            },
            ThemeKind::Acrylic => Theme { tint: Rgba::new(30, 34, 44, 140), ..dark },
            ThemeKind::Mica => Theme { tint: Rgba::new(28, 32, 42, 200), ..dark },
            ThemeKind::Glass => Theme { tint: Rgba::new(255, 255, 255, 28), ..dark },
            ThemeKind::Compact => Theme { minimal: true, ..dark },
        }
    }
}
