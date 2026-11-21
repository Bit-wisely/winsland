//! Island modes and their target geometry (logical pixels).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslandMode {
    Idle,
    Media,
    Charging,
    Notify,
    Focus,
    Expanded,
}

/// Target shape of the island inside the window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub w: f32,
    pub h: f32,
    pub radius: f32,
}

/// Fixed window size. Must be >= the largest shape; the island morphs inside it.
pub const WINDOW_W: f32 = 520.0;
pub const WINDOW_H: f32 = 200.0;
/// Gap between the screen top edge and the island.
pub const TOP_GAP: f32 = 8.0;

pub fn shape(mode: IslandMode) -> Shape {
    let pill = |w: f32, h: f32| Shape { w, h, radius: h / 2.0 };
    match mode {
        IslandMode::Idle => pill(126.0, 34.0),
        IslandMode::Media => pill(200.0, 34.0),
        IslandMode::Charging => pill(150.0, 34.0),
        IslandMode::Notify => pill(240.0, 44.0),
        IslandMode::Focus => pill(150.0, 34.0),
        IslandMode::Expanded => Shape { w: 460.0, h: 170.0, radius: 34.0 },
    }
}
