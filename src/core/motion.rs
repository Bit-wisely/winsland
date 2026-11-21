//! Spring presets per transition. Values are handed to the compositor,
//! which runs the animation GPU-side.
use super::island::IslandMode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub damping: f32,   // < 1.0 overshoots slightly
    pub period_ms: i64, // lower = faster
}

pub fn spring_for(from: IslandMode, to: IslandMode) -> Spring {
    match (from == IslandMode::Expanded, to == IslandMode::Expanded) {
        (false, true) => Spring { damping: 0.72, period_ms: 55 }, // expand: lively
        (true, false) => Spring { damping: 0.85, period_ms: 45 }, // collapse: settled
        _ => Spring { damping: 0.80, period_ms: 40 },             // pill <-> pill
    }
}
