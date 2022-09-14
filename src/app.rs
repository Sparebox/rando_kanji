use std::{cell::RefCell, rc::Rc, path::Path};

use egui_sfml::SfEgui;
use sfml::{graphics::{RenderWindow, Font, RenderTarget, Color, Text, View}, system::{Vector2f, Vector2u, Vector2i}, SfBox};

use crate::{config::Config, kanji::KanjiRecord, window::{ui::{TextDescriptor, TextButton, ButtonAction::{GotoGame, GotoOptions, GotoMenu, ExitGame, ToggleRomaji, ResetConfig}, AnswerData}, self, ViewEnum}, game_state::GameState, audio::{SoundPlayers, SoundBuffers}};
use crate::window::ui::ButtonAction::CheckAnswer;

pub struct App<'a> {
    pub window: RenderWindow,
    pub main_view: SfBox<View>,
    pub game_view: SfBox<View>,
    pub win_size: Vector2f,
    pub config: Config,
    pub kanjis: Vec<KanjiRecord>,
    pub font: sfml::SfBox<Font>,
    pub texts: Vec<TextDescriptor>,
    pub buttons: Rc<RefCell<Vec<TextButton<'a>>>>,
    pub current_state: GameState,
    pub is_switching_state: bool,
    pub sound_players: SoundPlayers<'a>,
    pub egui: SfEgui,
}

impl <'a>App<'a> {
    pub const FPS_LIMIT: u32 = 30;
    pub const FONT_SIZE: u32 = 50;
    pub const INIT_WIN_SIZE: Vector2u = Vector2u::new(1600, 900);
    pub const KANJI_DB_PATH: &'a str = "res/kanji_db.csv";
    pub const FONT_PATH: &'a str = "res/font/NotoSerifJP-Black.otf";
    pub const CONFIG_PATH: &'a str = "./config.json";

    pub fn new(sounds: &'a SoundBuffers) -> Self {
        let mut window = window::init();

        let main_view = View::new(
            Vector2f::new(App::INIT_WIN_SIZE.x as f32 / 2.0, App::INIT_WIN_SIZE.y as f32 / 2.0),
            Vector2f::new(window.size().x as f32, window.size().y as f32)
        );
        let game_view = main_view.clone();
        window.set_view(&main_view);

        let win_size = window.size().as_other();
        let config = 
            match Config::from_file(App::CONFIG_PATH) {
                Ok(config) => config,
                Err(_) => Config::default(),
            };

        let kanjis = KanjiRecord::from_csv(Path::new(App::KANJI_DB_PATH))
            .expect("Could not load kanjis");
        let font = Font::from_file(App::FONT_PATH)
            .expect("Could not load font");
        let texts = Vec::new();
        let buttons = Rc::new(RefCell::new(Vec::new()));
        let current_state = GameState::Menu;
        let is_switching_state = false;
        let sounds = SoundPlayers::new(sounds);
        let egui = SfEgui::new(&window);
        
        Self {
            window,
            main_view,
            game_view,
            win_size,
            config,
            kanjis,
            font,
            texts,
            buttons,
            current_state,
            is_switching_state,
            sound_players: sounds,
            egui,
        }
    }

    fn check_answer(&mut self, button: &mut TextButton, data: &AnswerData) {
        if data.index_to_test == data.correct_index { // If correct reading choice
            self.sound_players.correct_ans.play();
            let stat =
                self.config.answer_statistics.entry(data.kanji).or_insert(0);
            *stat += 1;
            self.change_state(GameState::Play);
        } else {
            self.sound_players.incorrect_ans.play();
            let stat = 
                self.config.answer_statistics.entry(data.kanji).or_insert(0);
            *stat -= 1;
            button.set_color(Color::RED, true);
        }
    }

    pub fn update_buttons(&mut self, mouse_pos: Vector2i, check_press: bool) {
        let mouse_pos = self.window.map_pixel_to_coords_current_view(mouse_pos);
        let mouse_pos = Vector2i::new(mouse_pos.x as i32, mouse_pos.y as i32);
        for button in self.buttons.clone().borrow_mut().iter_mut() {
            if button.get_width() > self.game_view.size().x {
                self.set_view_zoom(1.1);
            }
            if check_press {
                match button.check_for_mouse_press(mouse_pos) {
                    Some(GotoGame)     => self.change_state(GameState::Play),
                    Some(GotoOptions)  => self.change_state(GameState::Options),
                    Some(GotoMenu)     => self.change_state(GameState::Menu),
                    Some(CheckAnswer(data)) => self.check_answer(button, &data),
                    Some(ToggleRomaji) => { 
                        if self.config.romaji_enabled {
                            button.set_color(Color::WHITE, false);
                        } else {
                            button.set_color(Color::GREEN, true);
                        }
                        self.config.romaji_enabled = !self.config.romaji_enabled;
                    },
                    Some(ResetConfig) => {
                        self.config = Config::default();
                        self.change_state(GameState::Options);
                        button.set_color(Color::GREEN, true);
                    },
                    Some(ExitGame) => self.window.close(),
                    None => {},
                }
            } else {
                button.check_for_mouse_hover(mouse_pos);
            }
        }
    }

    pub fn change_state(&mut self, new_state: GameState) {
        self.is_switching_state = true;
        self.current_state = new_state;
    }

    pub fn draw(&mut self) {
        // Draw texts
        self.window.set_view(&self.main_view);
        let mut text = Text::new("", &self.font, 0);
        for t in self.texts.iter_mut() {
            t.as_sf_text(&mut text);
            self.window.draw(&text);
        }
        // Draw text buttons
        for button in self.buttons.borrow_mut().iter_mut() {
            match button.view_index {
                ViewEnum::GameButtonsView => self.window.set_view(&self.game_view),
                ViewEnum::DefaultView => self.window.set_view(&self.main_view),
            }
            button.draw(&mut self.window, &mut text);
        }
    }

    pub fn set_view_zoom(&mut self, factor: f32) {
        self.game_view.zoom(factor);
    }

    pub fn reset_zoom(&mut self) {
        self.game_view.set_size(Vector2f::new(self.window.size().x as f32, self.window.size().y as f32));
    }
}