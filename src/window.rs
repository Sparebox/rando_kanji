use sfml::window::{VideoMode, Window, ContextSettings, Style};

pub fn init() -> Window {
    
    Window::new(
        VideoMode::new(800, 600, 16),
        "Rust Game",
        Style::CLOSE,
        &ContextSettings::default(),
    )
}

