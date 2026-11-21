//! (state, event) -> new state + effects. No I/O, fully testable.
use super::event::Event;
use super::island::IslandMode;
use super::state::AppState;

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Morph { from: IslandMode, to: IslandMode },
    SetVisible(bool),
    Redraw,
    Quit,
}

pub fn reduce(s: &mut AppState, ev: Event) -> Vec<Effect> {
    let prev = s.mode;
    let mut fx = Vec::new();
    let expanded = s.mode == IslandMode::Expanded;

    match ev {
        Event::Click => {
            s.mode = if expanded { s.resting_mode() } else { IslandMode::Expanded };
        }
        Event::Hover(_) => {} // TODO(phase 4): subtle hover feedback
        Event::Escape => {
            if expanded {
                s.mode = s.resting_mode();
            } else {
                fx.push(Effect::Quit); // dev convenience until the tray menu exists
            }
        }
        Event::Quit => fx.push(Effect::Quit),

        Event::MediaChanged(m) => {
            s.media = m;
            rest_if_compact(s);
        }
        Event::BatteryChanged(b) => {
            s.battery = Some(b);
            rest_if_compact(s);
        }
        Event::NotificationPosted(n) => {
            s.notification = Some(n);
            rest_if_compact(s);
        }
        Event::NotificationDismissed => {
            s.notification = None;
            rest_if_compact(s);
        }
        Event::FocusRunning(r) => {
            s.focus_running = r;
            rest_if_compact(s);
        }
        Event::VolumeChanged(v) => {
            s.volume = v.clamp(0.0, 1.0);
            fx.push(Effect::Redraw);
        }
        Event::BrightnessChanged(v) => {
            s.brightness = v.clamp(0.0, 1.0);
            fx.push(Effect::Redraw);
        }
        Event::ConnectivityChanged(c) => {
            s.connectivity = c;
            fx.push(Effect::Redraw);
        }
        Event::StatsUpdated(st) => {
            s.stats = st;
            fx.push(Effect::Redraw);
        }
        Event::FullscreenChanged(f) => {
            s.hidden = f;
            fx.push(Effect::SetVisible(!f));
        }
    }

    if s.mode != prev {
        fx.push(Effect::Morph { from: prev, to: s.mode });
    }
    fx
}

fn rest_if_compact(s: &mut AppState) {
    if s.mode != IslandMode::Expanded {
        s.mode = s.resting_mode();
    }
}
