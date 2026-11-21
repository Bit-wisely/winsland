# Architecture

Data flows one way:

    services / window  ->  Event  ->  reducer(state)  ->  Effect  ->  compositor / renderer

- `core/` is pure and unit-tested; it never imports `windows` or `winit`.
- Services only send `Event`s through the winit `EventLoopProxy` (this wakes the loop, so
  there is no polling and no extra channel crate).
- `platform/compositor.rs` is the only owner of the Composition tree.
- `ui/` turns state into `DrawCmd`s; `render/` executes them (Direct2D, phase 3).
- The process sleeps (`ControlFlow::Wait`) between events; springs run inside DWM.

Window: fixed 520x200 logical px, topmost, borderless. The island shape morphs inside it
via a rounded-rect geometric clip. Click-through outside the shape: `platform/hittest.rs`
(phase 2).
