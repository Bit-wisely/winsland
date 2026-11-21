use winisland::core::event::*;
use winisland::core::island::IslandMode;
use winisland::core::reducer::{reduce, Effect};
use winisland::core::state::AppState;

fn media(playing: bool) -> MediaInfo {
    MediaInfo { title: "After Hours".into(), artist: "The Weeknd".into(), playing, position_s: 0, duration_s: 361 }
}

#[test]
fn click_toggles_expanded() {
    let mut s = AppState::default();
    let fx = reduce(&mut s, Event::Click);
    assert_eq!(s.mode, IslandMode::Expanded);
    assert_eq!(fx, vec![Effect::Morph { from: IslandMode::Idle, to: IslandMode::Expanded }]);
    reduce(&mut s, Event::Click);
    assert_eq!(s.mode, IslandMode::Idle);
}

#[test]
fn playing_media_shows_media_pill() {
    let mut s = AppState::default();
    reduce(&mut s, Event::MediaChanged(Some(media(true))));
    assert_eq!(s.mode, IslandMode::Media);
    reduce(&mut s, Event::MediaChanged(Some(media(false))));
    assert_eq!(s.mode, IslandMode::Idle);
}

#[test]
fn notification_beats_media_and_expanded_is_not_interrupted() {
    let mut s = AppState::default();
    reduce(&mut s, Event::MediaChanged(Some(media(true))));
    let n = Notification { app: "Outlook".into(), title: "New message".into(), body: "Hi".into() };
    reduce(&mut s, Event::NotificationPosted(n));
    assert_eq!(s.mode, IslandMode::Notify);

    reduce(&mut s, Event::Click); // expand
    reduce(&mut s, Event::NotificationDismissed);
    assert_eq!(s.mode, IslandMode::Expanded); // stays open
    reduce(&mut s, Event::Click); // collapse
    assert_eq!(s.mode, IslandMode::Media);
}

#[test]
fn escape_collapses_then_quits() {
    let mut s = AppState::default();
    reduce(&mut s, Event::Click);
    reduce(&mut s, Event::Escape);
    assert_eq!(s.mode, IslandMode::Idle);
    assert!(reduce(&mut s, Event::Escape).contains(&Effect::Quit));
}
