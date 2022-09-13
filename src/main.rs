use audio::{SoundBuffers, SoundPlayers};
use config::Config;
use game_state::{GameState::{self, Menu, Play, Options}};
use kanji::KanjiRecord;
use sfml::{
    SfBox,
    system::{Vector2f, Vector2u, Vector2i},
    graphics::{Color, Font, RenderTarget, RenderWindow, View, Text, Transformable, Shape},
};
use window::ui::{TextDescriptor, TextButton, ButtonAction::{GotoGame, GotoMenu, CheckAnswer, ExitGame, GotoOptions, ToggleRomaji}};
use std::{path::Path, cell::RefCell, rc::Rc};

mod kanji;
mod window;
mod game_state;
mod audio;
mod config;

pub struct App<'a> {
    window: RenderWindow,
    win_size: Vector2f,
    config: Config,
    kanjis: Vec<KanjiRecord>,
    font: SfBox<Font>,
    font_height: u32,
    texts: Vec<TextDescriptor>,
    buttons: Rc<RefCell<Vec<TextButton<'a>>>>,
    current_state: GameState,
    is_switching_state: bool,
    sound_players: SoundPlayers<'a>,
}

impl <'a>App<'a> {
    const FONT_MUL: f32 = 0.05;
    const FPS_LIMIT: u32 = 60;
    const INIT_WIN_SIZE: Vector2u = Vector2u::new(1800, 1200);
    const CONFIG_PATH: &'a str = "./config.json";

    fn new(sounds: &'a SoundBuffers) -> Self {
        let window = window::init();
        let win_size = window.size().as_other();
        let config = 
            match Config::from_file(App::CONFIG_PATH) {
                Ok(config) => config,
                Err(_) => Config::default(),
            };
        let kanjis = KanjiRecord::from_csv(Path::new("res/kanji_db.csv"))
            .expect("Could not load kanjis");
        let font = Font::from_file("res/font/NotoSerifJP-Black.otf")
            .expect("Could not load font");
        let font_height = (App::FONT_MUL * win_size.y) as u32;
        let texts = Vec::new();
        let buttons = Rc::new(RefCell::new(Vec::new()));
        let current_state = GameState::Menu;
        let is_switching_state = false;
        let sounds = SoundPlayers::new(sounds);
        
        Self {
            window,
            win_size,
            config,
            kanjis,
            font,
            font_height,
            texts,
            buttons,
            current_state,
            is_switching_state,
            sound_players: sounds,
        }
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        let view = View::new(
            Vector2f::new(width / 2.0, height / 2.0),
            Vector2f::new(width, height)
        );
        self.window.set_view(&view);
        self.font_height = (App::FONT_MUL * height) as u32;
        for text in self.texts.iter_mut() {
            let x_percentage = text.pos.x / self.win_size.x;
            let y_percentage = text.pos.y / self.win_size.y;
            text.pos.x = x_percentage * width;
            text.pos.y = y_percentage * height;
        }

        for button in self.buttons.borrow_mut().iter_mut() {
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

    fn update_buttons(&mut self, mouse_pos: Vector2i, check_press: bool) {
        for button in self.buttons.clone().borrow_mut().iter_mut() {
            if check_press {
                match button.check_for_mouse_press(mouse_pos) {
                    Some(GotoGame)     => self.change_state(GameState::Play),
                    Some(GotoOptions)  => self.change_state(GameState::Options),
                    Some(GotoMenu)     => self.change_state(GameState::Menu),
                    Some(CheckAnswer(data)) => { 
                        if data.index_to_test == data.correct_index { // If correct reading choice
                            self.sound_players.correct_ans.play();
                            self.change_state(GameState::Play);
                        } else {
                            self.sound_players.incorrect_ans.play();
                            button.set_color(Color::RED, true);
                        }
                    },
                    Some(ToggleRomaji) => { 
                        if self.config.romaji_enabled {
                            button.set_color(Color::WHITE, false);
                        } else {
                            button.set_color(Color::GREEN, true);
                        }
                        self.config.romaji_enabled = !self.config.romaji_enabled;
                    },
                    Some(ExitGame) => self.window.close(),
                    None => {},
                }
            } else {
                button.check_for_mouse_hover(mouse_pos);
            }
        }
    }

    fn change_state(&mut self, new_state: GameState) {
        self.is_switching_state = true;
        self.current_state = new_state;
    }

    fn draw(&mut self) {
        // Draw texts
        let mut text = Text::new("", &self.font, 0);
        for t in self.texts.iter_mut() {
            t.as_sf_text(&mut text, self.font_height);
            self.window.draw(&text);
        }
        // Draw text buttons
        for button in self.buttons.borrow_mut().iter_mut() {
            self.window.draw(&button.shape);
            button.text.as_sf_text(&mut text, self.font_height);
            self.window.draw(&text);
        }
    }
}

fn main() {
    let sounds = SoundBuffers::new();
    let mut app = App::new(&sounds);
    app.change_state(GameState::Menu);
    
    // Update loop
    while app.window.is_open() {
        if app.is_switching_state {
            app.is_switching_state = false;
            match app.current_state {
                Menu => { GameState::init_menu_state(&mut app) },
                Options => { GameState::init_options_state(&mut app) },
                Play => { GameState::init_play_state(&mut app) },
            }
        }
        app.window.clear(Color::BLACK);
        window::handle_events(&mut app);
        app.draw();
        app.window.display();
    }
    // Save configurations to disk
    app.config.to_file(App::CONFIG_PATH);
}

