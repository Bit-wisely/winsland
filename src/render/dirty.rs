//! Track which parts of the UI changed so only those are redrawn.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Dirty(u32);

impl Dirty {
    pub const NONE: Dirty = Dirty(0);
    pub const COMPACT: Dirty = Dirty(1);
    pub const CONTENT: Dirty = Dirty(1 << 1);
    pub const ALL: Dirty = Dirty(u32::MAX);

    pub fn mark(&mut self, other: Dirty) {
        self.0 |= other.0;
    }
    pub fn contains(&self, other: Dirty) -> bool {
        self.0 & other.0 != 0
    }
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
