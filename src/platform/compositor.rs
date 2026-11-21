//! Owns the Windows.UI.Composition tree:
//!   root -> island (rounded geometric clip) -> [backdrop blur, tint]
//! Springs run in the DWM compositor, so the app thread stays idle while animating.
use std::mem::size_of;

use windows::{
    core::{Interface, Result, HSTRING},
    Foundation::{Numerics::Vector2, TimeSpan},
    System::DispatcherQueueController,
    UI::{
        Color,
        Composition::{
            CompositionAnimation, CompositionBrush, CompositionClip, CompositionObject,
            CompositionRoundedRectangleGeometry, Compositor, Desktop::DesktopWindowTarget, Visual,
        },
    },
    Win32::{
        Foundation::HWND,
        System::WinRT::{
            Composition::ICompositorDesktopInterop, CreateDispatcherQueueController,
            DispatcherQueueOptions, RoInitialize, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
            RO_INIT_SINGLETHREADED,
        },
    },
};

use crate::core::island::{Shape, TOP_GAP, WINDOW_W};
use crate::core::motion::Spring;
use crate::render::cmd::Rgba;

pub struct IslandCompositor {
    _queue: DispatcherQueueController,
    compositor: Compositor,
    _target: DesktopWindowTarget,
    geometry: CompositionRoundedRectangleGeometry,
    scale: f32,
}

fn v2(x: f32, y: f32) -> Vector2 {
    Vector2 { X: x, Y: y }
}

impl IslandCompositor {
    pub fn new(hwnd: HWND, scale: f32, tint: Rgba, use_backdrop: bool) -> Result<Self> {
        unsafe {
            let _ = RoInitialize(RO_INIT_SINGLETHREADED); // winit usually did this already

            let queue = CreateDispatcherQueueController(DispatcherQueueOptions {
                dwSize: size_of::<DispatcherQueueOptions>() as u32,
                threadType: DQTYPE_THREAD_CURRENT,
                apartmentType: DQTAT_COM_NONE,
            })?;

            let compositor = Compositor::new()?;
            let interop: ICompositorDesktopInterop = compositor.cast()?;
            let target = interop.CreateDesktopWindowTarget(hwnd, true)?;

            let root = compositor.CreateContainerVisual()?;
            root.SetRelativeSizeAdjustment(v2(1.0, 1.0))?;
            target.SetRoot(&root.cast::<Visual>()?)?;

            let island = compositor.CreateContainerVisual()?;
            island.SetRelativeSizeAdjustment(v2(1.0, 1.0))?;
            root.Children()?.InsertAtTop(&island.cast::<Visual>()?)?;

            if use_backdrop {
                let backdrop = compositor.CreateHostBackdropBrush()?;
                let sprite = compositor.CreateSpriteVisual()?;
                sprite.SetRelativeSizeAdjustment(v2(1.0, 1.0))?;
                sprite.SetBrush(&backdrop.cast::<CompositionBrush>()?)?;
                island.Children()?.InsertAtTop(&sprite.cast::<Visual>()?)?;
            }

            let brush = compositor.CreateColorBrushWithColor(Color {
                A: tint.a,
                R: tint.r,
                G: tint.g,
                B: tint.b,
            })?;
            let tint_sprite = compositor.CreateSpriteVisual()?;
            tint_sprite.SetRelativeSizeAdjustment(v2(1.0, 1.0))?;
            tint_sprite.SetBrush(&brush.cast::<CompositionBrush>()?)?;
            island.Children()?.InsertAtTop(&tint_sprite.cast::<Visual>()?)?;

            let geometry = compositor.CreateRoundedRectangleGeometry()?;
            let clip = compositor.CreateGeometricClipWithGeometry(&geometry)?;
            island.SetClip(&clip.cast::<CompositionClip>()?)?;

            // TODO(phase 3): add a content SpriteVisual here backed by a D2D surface.

            Ok(Self { _queue: queue, compositor, _target: target, geometry, scale })
        }
    }

    /// Physical-pixel (size, offset, corner radius) for a logical shape.
    fn physical(&self, s: Shape) -> (Vector2, Vector2, Vector2) {
        let k = self.scale;
        let (w, h, r) = (s.w * k, s.h * k, s.radius * k);
        (v2(w, h), v2((WINDOW_W * k - w) / 2.0, TOP_GAP * k), v2(r, r))
    }

    /// `None` jumps instantly; `Some(spring)` morphs. A new spring on a
    /// property replaces the running one, which keeps animations interruptible.
    pub fn apply_shape(&self, shape: Shape, spring: Option<Spring>) -> Result<()> {
        let (size, offset, radius) = self.physical(shape);
        match spring {
            Some(sp) => {
                self.spring("Size", size, sp)?;
                self.spring("Offset", offset, sp)?;
                self.spring("CornerRadius", radius, sp)?;
            }
            None => {
                self.geometry.SetSize(size)?;
                self.geometry.SetOffset(offset)?;
                self.geometry.SetCornerRadius(radius)?;
            }
        }
        Ok(())
    }

    fn spring(&self, property: &str, to: Vector2, sp: Spring) -> Result<()> {
        let anim = self.compositor.CreateSpringVector2Animation()?;
        anim.SetFinalValue(to)?;
        anim.SetDampingRatio(sp.damping)?;
        anim.SetPeriod(TimeSpan { Duration: sp.period_ms * 10_000 })?; // 100 ns ticks
        let obj: CompositionObject = self.geometry.cast()?;
        let anim: CompositionAnimation = anim.cast()?;
        obj.StartAnimation(&HSTRING::from(property), &anim)?;
        Ok(())
    }
}
