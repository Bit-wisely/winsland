//! Wires window events and service events into the reducer, and applies the
//! resulting effects to the compositor.
use std::error::Error;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

use crate::config::settings::Settings;
use crate::core::event::Event;
use crate::core::island;
use crate::core::motion;
use crate::core::reducer::{reduce, Effect};
use crate::core::state::AppState;
use crate::platform::{compositor::IslandCompositor, window};
use crate::services::{self, Service, Tx};
use crate::ui::theme::{Backdrop, Theme};

struct App {
    proxy: Tx,
    settings: Settings,
    state: AppState,
    window: Option<Window>,
    comp: Option<IslandCompositor>,
    services: Vec<Box<dyn Service>>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let settings = Settings::load();
    let el = EventLoop::<Event>::with_user_event().build()?;
    el.set_control_flow(ControlFlow::Wait); // sleep when idle
    let mut app = App {
        proxy: el.create_proxy(),
        settings,
        state: AppState::default(),
        window: None,
        comp: None,
        services: Vec::new(),
    };
    el.run_app(&mut app)?;
    Ok(())
}

impl App {
    fn init(&mut self, el: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let w = window::create(el)?;
        let theme = Theme::for_kind(self.settings.theme);
        let use_backdrop = theme.backdrop == Backdrop::HostAcrylic && !self.settings.force_solid_backdrop;
        let comp = IslandCompositor::new(w.hwnd, w.scale, theme.tint, use_backdrop)?;
        comp.apply_shape(island::shape(self.state.mode), None)?;
        self.comp = Some(comp);
        self.window = Some(w.window);
        self.services = services::start_all(&self.proxy, &self.settings);
        Ok(())
    }

    fn dispatch(&mut self, el: &ActiveEventLoop, ev: Event) {
        for fx in reduce(&mut self.state, ev) {
            match fx {
                Effect::Morph { from, to } => {
                    if let Some(c) = &self.comp {
                        let spring = motion::spring_for(from, to);
                        if let Err(e) = c.apply_shape(island::shape(to), Some(spring)) {
                            tracing::error!("morph failed: {e}");
                        }
                    }
                }
                Effect::SetVisible(v) => {
                    if let Some(w) = &self.window {
                        w.set_visible(v);
                    }
                }
                Effect::Redraw => {} // TODO(phase 3): render ui::* into the D2D surface
                Effect::Quit => el.exit(),
            }
        }
    }
}

impl ApplicationHandler<Event> for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        if let Err(e) = self.init(el) {
            tracing::error!("init failed: {e}");
            eprintln!("winisland init failed: {e}");
            el.exit();
        }
    }

    fn user_event(&mut self, el: &ActiveEventLoop, ev: Event) {
        self.dispatch(el, ev);
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        let mapped = match ev {
            WindowEvent::CloseRequested => Some(Event::Quit),
            WindowEvent::CursorEntered { .. } => Some(Event::Hover(true)),
            WindowEvent::CursorLeft { .. } => Some(Event::Hover(false)),
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                Some(Event::Click)
            }
            WindowEvent::KeyboardInput { event, .. }
                if event.state == ElementState::Pressed
                    && event.logical_key == Key::Named(NamedKey::Escape) =>
            {
                Some(Event::Escape)
            }
            _ => None,
        };
        if let Some(e) = mapped {
            self.dispatch(el, e);
        }
    }
}
