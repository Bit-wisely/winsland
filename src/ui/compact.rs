//! Pill contents for each compact mode.
use crate::core::island::IslandMode;
use crate::core::state::AppState;
use crate::render::cmd::DrawCmd;
use crate::render::layout::Rect;
use crate::ui::theme::Theme;

pub fn build(state: &AppState, area: Rect, theme: &Theme, out: &mut Vec<DrawCmd>) {
    match state.mode {
        IslandMode::Idle => {
            // Two dots, as in the design mockup.
            let (cx, cy) = area.center();
            for dx in [-6.0, 6.0] {
                out.push(DrawCmd::Circle { cx: cx + dx, cy, r: 3.0, color: theme.text_primary });
            }
        }
        // TODO(phase 3/5/6): artwork + waveform, battery ring, bell, timer
        _ => {}
    }
}
