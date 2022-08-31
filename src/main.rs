use kanji::Kanji;
use sfml::{
    system::{Vector2f},
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    window::{Event, Key},
};
use std::path::Path;

mod kanji;
mod window;

pub struct App {
    window: RenderWindow,
    kanjis: Vec<Kanji>,
    font: sfml::SfBox<Font>,
}

impl App {
    fn new() -> Self {
        let window = window::init();
        let kanjis = Kanji::from_csv(Path::new("res/kanji_db.csv")).expect("Could not load kanjis");
        let font = Font::from_file("res/font/NotoSerifJP-Black.otf").expect("Could not load font");
        Self {
            window,
            kanjis,
            font,
        }
    }
}

fn main() {
    let mut app = App::new();

    while app.window.is_open() {
        window::handle_events(&mut app);
        app.window.display();
    }
}
