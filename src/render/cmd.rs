//! What the UI layer produces and the renderer executes.
use super::layout::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawCmd {
    RoundRect { rect: Rect, radius: f32, color: Rgba },
    Circle { cx: f32, cy: f32, r: f32, color: Rgba },
    Text { text: String, rect: Rect, size: f32, weight: u16, color: Rgba },
    Icon { glyph: char, rect: Rect, size: f32, color: Rgba },
    ProgressBar { rect: Rect, value: f32, track: Rgba, fill: Rgba },
}
