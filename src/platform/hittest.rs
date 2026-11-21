//! Click-through outside the island shape.
//! Pure geometry here; the WM_NCHITTEST window subclass that calls it is TODO(phase 2).

/// Is point (px, py) inside a rounded rect at (x, y) of size (w, h) with corner radius r?
pub fn inside_rounded_rect(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, r: f32) -> bool {
    if px < x || py < y || px > x + w || py > y + h {
        return false;
    }
    let r = r.min(w / 2.0).min(h / 2.0);
    let cx = px.clamp(x + r, x + w - r);
    let cy = py.clamp(y + r, y + h - r);
    let (dx, dy) = (px - cx, py - cy);
    dx * dx + dy * dy <= r * r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_are_cut() {
        assert!(inside_rounded_rect(50.0, 20.0, 0.0, 0.0, 100.0, 40.0, 20.0));
        assert!(!inside_rounded_rect(1.0, 1.0, 0.0, 0.0, 100.0, 40.0, 20.0));
        assert!(!inside_rounded_rect(150.0, 20.0, 0.0, 0.0, 100.0, 40.0, 20.0));
    }
}
