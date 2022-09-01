use kanji::Kanji;
use sfml::{
    SfBox,
    system::Vector2f,
    graphics::{Color, Font, RenderTarget, RenderWindow, Transformable, View}
};
use window::TextDescriptor;
use std::path::Path;
use rand::prelude::*;

mod kanji;
mod window;

pub struct App {
    window: RenderWindow,
    win_size: Vector2f,
    kanjis: Vec<Kanji>,
    font: SfBox<Font>,
    font_height: u32,
    texts: Vec<TextDescriptor>,
}

impl App {
    const FONT_MUL:f32 = 0.09;

    fn new() -> Self {
        let window = window::init();
        let win_size = window.size().as_other();
        let kanjis = Kanji::from_csv(Path::new("res/kanji_db.csv")).expect("Could not load kanjis");
        let font = Font::from_file("res/font/NotoSerifJP-Black.otf").expect("Could not load font");
        let font_height = (App::FONT_MUL * win_size.y) as u32;
        let texts = Vec::new();
        Self {
            window,
            win_size,
            kanjis,
            font,
            font_height,
            texts,
        }
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        let view = View::new(Vector2f::new(width / 2.0, height / 2.0), Vector2f::new(width, height));
        self.window.set_view(&view);

        for text in self.texts.iter_mut() {
            let width_percentage = text.pos.x / self.win_size.x;
            text.pos.x = width_percentage * width;
        }
        self.win_size = Vector2f::new(width, height);
        self.font_height = (App::FONT_MUL * height) as u32;
        self.texts.clear();
        create_menu(self);
    }

}

fn main() {
    let mut app = App::new();
    create_menu(&mut app);

    while app.window.is_open() {
        app.window.clear(Color::BLACK);
        window::handle_events(&mut app);

        // Draw texts
        let mut text = sfml::graphics::Text::new("", &app.font, 0);
        for t in app.texts.iter() {
            text.set_string(&t.string);
            text.set_position(t.pos);
            text.set_character_size(t.font_base_size + app.font_height);
            if t.center {
                let width = text.global_bounds().width;
                let height = text.global_bounds().height;
                text.move_(Vector2f::new(-width / 2.0, -height / 2.0));
            }
            app.window.draw(&text);
        }
        app.window.display();
    }
}

fn create_menu(app: &mut App) {
    let height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
    let mut text = TextDescriptor::new("Rando Kanji ・ ランド漢字", Vector2f::new(app.window.size().x as f32 / 2.0, height_offset), Color::WHITE, true);
    text.font_base_size = 0;
    app.texts.push(text.clone());

    text.string = String::from("Play");
    text.pos.y += height_offset + 20.0;
    app.texts.push(text.clone());

    text.string = String::from("Exit");
    text.pos.y += height_offset;
    app.texts.push(text);
}
