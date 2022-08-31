use sfml::{window::{VideoMode, ContextSettings, Style, Event, Key}, graphics::RenderWindow};

use crate::App;

pub fn init() -> RenderWindow {
    RenderWindow::new(
        VideoMode::new(800, 600, 16),
        "Rust Game",
        Style::CLOSE,
        &ContextSettings::default(),
    )
}

pub fn handle_events(app: &mut App) {
    while let Some(event) = app.window.poll_event() {
        match event {
            Event::TextEntered { unicode } => {
                println!("{unicode}")
            }
            Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => app.window.close(),
            _ => { /* Do nothing */ }
        }
    }
}

