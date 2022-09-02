use kanji::Kanji;
use sfml::{
    SfBox,
    system::{Vector2f, Vector2u},
    graphics::{Color, Font, RenderTarget, RenderWindow, Transformable, View, Text}
};
use window::ui::{TextDescriptor, TextButton};
use std::path::Path;
use rand::prelude::*;

mod kanji;
mod window;

pub struct App<'a> {
    window: RenderWindow,
    win_size: Vector2f,
    kanjis: Vec<Kanji>,
    font: SfBox<Font>,
    font_height: u32,
    texts: Vec<TextDescriptor>,
    buttons: Vec<TextButton<'a>>
}

impl <'a>App<'a> {
    const FONT_MUL:f32 = 0.09;
    const FPS_LIMIT:u32 = 60;
    const INIT_WIN_SIZE:Vector2u = Vector2u::new(900, 600);

    fn new() -> Self {
        let window = window::init();
        let win_size = window.size().as_other();
        let kanjis = Kanji::from_csv(Path::new("res/kanji_db.csv")).expect("Could not load kanjis");
        let font = Font::from_file("res/font/NotoSerifJP-Black.otf").expect("Could not load font");
        let font_height = (App::FONT_MUL * win_size.y) as u32;
        let texts = Vec::new();
        let buttons = Vec::new();
        Self {
            window,
            win_size,
            kanjis,
            font,
            font_height,
            texts,
            buttons,
        }
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        let view = View::new(Vector2f::new(width / 2.0, height / 2.0), Vector2f::new(width, height));
        self.window.set_view(&view);
        self.font_height = (App::FONT_MUL * height) as u32;

        for text in self.texts.iter_mut() {
            let x_percentage = text.pos.x / self.win_size.x;
            let y_percentage = text.pos.y / self.win_size.y;
            text.pos.x = x_percentage * width;
            text.pos.y = y_percentage * height;
        }

        for button in self.buttons.iter_mut() {
            let mut pos = button.shape.position();
            let mut size = button.shape.size();
            let aspect = size.x / size.y;
            let x_percentage = pos.x / self.win_size.x;
            let y_percentage = pos.y / self.win_size.y;
            let height_percentage = size.y / self.win_size.y;
            pos.x = x_percentage * width;
            pos.y = y_percentage * height;
            size.y = height_percentage * height;
            size.x = aspect * size.y;
            button.shape.set_position(pos);
            button.shape.set_size(size);
            button.text.pos = pos + size / 2.0;
        }
        self.win_size = Vector2f::new(width, height);
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
        for t in app.texts.iter_mut() {
            text_from_descriptor(&mut text, t, app.font_height);
            app.window.draw(&text);
        }
        // Draw text buttons
        for button in app.buttons.iter_mut() {
            app.window.draw(&button.shape);
            text_from_descriptor(&mut text, &mut button.text, app.font_height);
            app.window.draw(&text);
        }
        app.window.display();
    }
}

fn create_menu(app: &mut App) {
    let height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
    let mut title = TextDescriptor::new("Rando Kanji ・ ランド漢字", Vector2f::new(app.window.size().x as f32 / 2.0, height_offset), Color::WHITE, true);
    title.font_base_size = 10;
    app.texts.push(title);

    let mut button = TextButton::new(
        "Play",
        Vector2f::new(app.window.size().x as f32 / 2.0, height_offset + 100.0), 
        Color::WHITE,
        Color::WHITE,
        app,
        |app| {},
    );
    app.buttons.push(button.clone());

    button = TextButton::new(
        "Exit",
        Vector2f::new(app.window.size().x as f32 / 2.0, height_offset + 200.0),
        Color::WHITE,
        Color::WHITE,
        app,
        |app| app.window.close(),
    );
    app.buttons.push(button);
}

fn text_from_descriptor(sf_text: &mut Text, descriptor: &mut TextDescriptor, font_height: u32) {
    sf_text.set_string(&descriptor.string);
    sf_text.set_position(descriptor.pos);
    sf_text.set_fill_color(descriptor.color);
    sf_text.set_character_size(descriptor.font_base_size + font_height);
    descriptor.bounds = sf_text.global_bounds();
    if descriptor.center {
        let width = sf_text.global_bounds().width;
        let height = sf_text.global_bounds().height;
        sf_text.move_(Vector2f::new(-width / 2.0, -height / 2.0));
    }
}
