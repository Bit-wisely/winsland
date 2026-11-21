//! Tiny layout helpers. ~6 screens don't need a framework.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn inset(&self, d: f32) -> Rect {
        Rect::new(self.x + d, self.y + d, (self.w - 2.0 * d).max(0.0), (self.h - 2.0 * d).max(0.0))
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    /// Split into a row. A weight of 0.0 means "use `fixed`"; others share the rest.
    /// Example: `r.split_row(&[(0.0, 40.0), (1.0, 0.0), (1.0, 0.0)], 8.0)`
    pub fn split_row(&self, items: &[(f32, f32)], gap: f32) -> Vec<Rect> {
        let gaps = gap * items.len().saturating_sub(1) as f32;
        let fixed: f32 = items.iter().filter(|(w, _)| *w == 0.0).map(|(_, f)| *f).sum();
        let total_w: f32 = items.iter().map(|(w, _)| *w).sum();
        let free = (self.w - gaps - fixed).max(0.0);
        let mut x = self.x;
        items
            .iter()
            .map(|(weight, f)| {
                let w = if *weight == 0.0 { *f } else { free * weight / total_w };
                let r = Rect::new(x, self.y, w, self.h);
                x += w + gap;
                r
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_row_shares_free_space() {
        let r = Rect::new(0.0, 0.0, 300.0, 40.0);
        let parts = r.split_row(&[(0.0, 40.0), (1.0, 0.0), (1.0, 0.0)], 10.0);
        assert_eq!(parts[0].w, 40.0);
        assert_eq!(parts[1].w, 120.0);
        assert_eq!(parts[2].x, 40.0 + 10.0 + 120.0 + 10.0);
    }
}
