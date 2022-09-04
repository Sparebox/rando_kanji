use game_state::{GameState, MenuState, StateEnum::{self, Menu, Play}, PlayState};
use kanji::Kanji;
use sfml::{
    SfBox,
    system::{Vector2f, Vector2u},
    graphics::{Color, Font, RenderTarget, RenderWindow, View}
};
use std::path::Path;
use rand::prelude::*;

mod kanji;
mod window;
mod game_state;

pub struct App {
    window: RenderWindow,
    win_size: Vector2f,
    kanjis: Vec<Kanji>,
    font: SfBox<Font>,
    font_height: u32,
    next_state: StateEnum,
    is_switching_state: bool,
}

impl App {
    const FONT_MUL:f32 = 0.09;
    const FPS_LIMIT:u32 = 60;
    const INIT_WIN_SIZE:Vector2u = Vector2u::new(900, 600);

    fn new() -> Self {
        let window = window::init();
        let win_size = window.size().as_other();
        let kanjis = Kanji::from_csv(Path::new("res/kanji_db.csv")).expect("Could not load kanjis");
        let font = Font::from_file("res/font/NotoSerifJP-Black.otf").expect("Could not load font");
        let font_height = (App::FONT_MUL * win_size.y) as u32;
        let next_state = StateEnum::Menu;
        let is_switching_state = true;
        
        Self {
            window,
            win_size,
            kanjis,
            font,
            font_height,
            next_state,
            is_switching_state,
        }
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        let view = View::new(Vector2f::new(width / 2.0, height / 2.0), Vector2f::new(width, height));
        self.window.set_view(&view);
        self.font_height = (App::FONT_MUL * height) as u32;
        self.win_size = Vector2f::new(width, height);
    }

    fn change_state(&mut self, new_state: StateEnum) {
        self.is_switching_state = true;
        self.next_state = new_state;
    }

}

fn main() {
    let mut app = App::new();
    let mut game_state: Box<dyn GameState> = Box::new(MenuState::new(&app));
    app.change_state(StateEnum::Menu);
    while app.window.is_open() {
        if app.is_switching_state {
            app.is_switching_state = false;
            match app.next_state {
                Menu => {game_state = Box::new(MenuState::new(&app))},
                Play => {game_state = Box::new(PlayState::new())},
            }
        }
        app.window.clear(Color::BLACK);
        window::handle_events(&mut app, &mut game_state);
        game_state.update(&mut app);
        game_state.draw(&mut app);
    }
}

