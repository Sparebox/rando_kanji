use sfml::{
    graphics::{Color, RenderWindow},
    system::Vector2f,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

use crate::App;

pub fn init() -> RenderWindow {
    let settings = ContextSettings {
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600, 16),
        "Rando Kanji ・ ランド漢字",
        Style::DEFAULT,
        &settings,
    );
    window.set_framerate_limit(60);
    window
}

pub fn handle_events(app: &mut App) {
    while let Some(event) = app.window.poll_event() {
        match event {
            Event::Resized { width, height } => app.on_resize(width as f32, height as f32),
            Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => app.window.close(),
            _ => { /* Do nothing */ }
        }
    }
}

#[derive(Clone)]
pub struct TextDescriptor {
    pub string: String,
    pub pos: Vector2f,
    pub color: Color,
    pub font_base_size: u32,
    pub center: bool,
}

impl TextDescriptor {
    pub fn new(string: &str, pos: Vector2f, color: Color, center: bool) -> Self {
        Self {
            string: string.to_string(),
            pos,
            color,
            font_base_size: 0,
            center,
        }
    }
}
