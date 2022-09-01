use sfml::{
    graphics::RenderWindow,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

use crate::App;

pub fn init() -> RenderWindow {
    let settings = ContextSettings {
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        VideoMode::new(App::INIT_WIN_SIZE.x, App::INIT_WIN_SIZE.y, 16),
        "Rando Kanji ・ ランド漢字",
        Style::DEFAULT,
        &settings,
    );
    window.set_framerate_limit(App::FPS_LIMIT);
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

pub mod ui {
    use sfml::{system::Vector2f, graphics::{Color, RectangleShape, Transformable, Shape, Text}};

    use crate::App;

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

    #[derive(Clone)]
    pub struct TextButton<'a> {
        pub text: TextDescriptor,
        pub shape: RectangleShape<'a>
    }

    impl <'a>TextButton<'a> {
        pub fn new(string: &str, pos: Vector2f, fg_color: Color, bg_color: Color, app: &App) -> Self {
            let text = TextDescriptor::new(string, pos, fg_color, true);
            let mut text_bounds = Text::new(string, &app.font, app.font_height).global_bounds();
            text_bounds.width += app.font_height as f32 / 2.0;
            text_bounds.height += app.font_height as f32 / 2.0;
            let mut shape = RectangleShape::from_rect(text_bounds);
            shape.set_position(pos - shape.size() / 2.0);
            shape.set_outline_color(bg_color);
            shape.set_outline_thickness(1.0);
            shape.set_fill_color(Color::TRANSPARENT);
            Self {
                text,
                shape
            }
        }
    }
}