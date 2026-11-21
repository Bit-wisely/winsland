//! Drawing. `cmd` and `layout` are plain data (cross-platform);
//! `d2d`, `text`, `icons` do the Windows-side work in later phases.
pub mod cmd;
pub mod d2d;
pub mod dirty;
pub mod icons;
pub mod layout;
pub mod text;
