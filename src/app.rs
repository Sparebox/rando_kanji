use std::{cell::RefCell, rc::Rc};

use egui_sfml::SfEgui;
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, View},
    system::{Vector2f, Vector2i, Vector2u},
    SfBox,
};

use crate::{
    audio::{SoundBuffers, SoundPlayers},
    config::Config,
    game_state::GameState,
    window::{
        self,
        ui::{self, AnswerData, TextButton, TextDescriptor},
    },
};
use crate::{config::StatValue, kanji::KanjiDealer, utils, window::ui::ButtonAction::CheckAnswer};

pub struct App<'a> {
    pub window: RenderWindow,
    pub main_view: SfBox<View>,
    pub game_view: SfBox<View>,
    pub config: Config,
    pub kanji_dealer: KanjiDealer,
    pub font: SfBox<Font>,
    pub texts: Vec<TextDescriptor>,
    pub buttons: Rc<RefCell<Vec<TextButton<'a>>>>,
    pub current_state: GameState,
    pub is_switching_state: bool,
    pub sound_players: SoundPlayers<'a>,
    pub egui: SfEgui,
    pub showing_confirm_dialog: bool,
}

impl<'a> App<'a> {
    pub const FPS_LIMIT: u32 = 30;
    pub const FONT_SIZE: u32 = 50;
    pub const INIT_WIN_SIZE: Vector2u = Vector2u::new(1600, 900);
    pub const BACKGROUND_COLOR: Color = Color::rgb(10, 10, 10);
    pub const KANJI_DB_PATH: &'static str = "res/kanji_db.csv";
    pub const FONT_PATH: &'static str = "res/font/Honoka-Shin-Antique-Maru_R.otf";
    pub const CONFIG_PATH: &'static str = "./config";
    pub const CONFIG_FILE_EXTENSION: &'static str = ".json";

    pub fn new(sounds: &'a SoundBuffers) -> Self {
        let mut window = window::init();

        let main_view = View::new(
            Vector2f::new(
                App::INIT_WIN_SIZE.x as f32 / 2.0,
                App::INIT_WIN_SIZE.y as f32 / 2.0,
            ),
            Vector2f::new(window.size().x as f32, window.size().y as f32),
        );
        let game_view = main_view.clone();
        window.set_view(&main_view);

        let config = Self::load_config();
        let kanji_dealer = KanjiDealer::new();
        let font = Font::from_file(App::FONT_PATH).expect("Could not load font");
        let texts = Vec::new();
        let buttons = Rc::new(RefCell::new(Vec::new()));
        let current_state = GameState::Menu;
        let is_switching_state = false;
        let sounds = SoundPlayers::new(sounds);
        let egui = SfEgui::new(&window);
        ui::set_custom_egui_font(egui.context());

        Self {
            window,
            main_view,
            game_view,
            config,
            kanji_dealer,
            font,
            texts,
            buttons,
            current_state,
            is_switching_state,
            sound_players: sounds,
            egui,
            showing_confirm_dialog: false,
        }
    }

    fn load_config() -> Config {
        for i in 1..=3 {
            if let Ok(mut config) = Config::from_file(format!("{}{}{}", App::CONFIG_PATH, i, App::CONFIG_FILE_EXTENSION).as_str()) {
                config.reset_review_times();
                return config;
            }
        }
        eprintln!("Could not load config from file");
        Config::default()
    }

    fn check_answer(&mut self, button: &mut TextButton, ans_data: &AnswerData) {
        if ans_data.index_to_test == ans_data.correct_index {
            // If correct reading choice
            self.sound_players.correct_ans.play();
            let entry = self
                .config
                .answer_statistics
                .entry(ans_data.kanji)
                .or_insert_with(StatValue::default);
            entry.learning_index += 1;
            if entry.learning_index >= self.config.learning_index_threshold {
                entry.review_interval += Config::REVIEW_INTERVAL_STEP;
            }
            self.change_state(GameState::Play); // Show a new kanji
        } else {
            // Incorrect reading choice
            self.sound_players.incorrect_ans.play();
            let entry = self
                .config
                .answer_statistics
                .entry(ans_data.kanji)
                .or_insert_with(StatValue::default);
            entry.learning_index -= 1;
            button.set_color(Color::RED, true);
        }
    }

    pub fn update_buttons(&mut self, mouse_pos: Vector2i, check_press: bool) {
        let mapped_mouse_pos = utils::vector2f_to_vector2i(
            self.window.map_pixel_to_coords(mouse_pos, &self.game_view),
        );

        for button in self.buttons.clone().borrow_mut().iter_mut() {
            // Check if a button overlaps the window and zoom out accordingly
            if button.get_width() > self.game_view.size().x {
                self.set_view_zoom(1.1);
            }
            if check_press {
                match button.check_for_mouse_press(mapped_mouse_pos) {
                    Some(CheckAnswer(data)) => self.check_answer(button, &data),
                    None => {}
                }
            } else {
                button.check_for_mouse_hover(mapped_mouse_pos);
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
        // Draw game text buttons
        for button in self.buttons.borrow_mut().iter_mut() {
            self.window.set_view(&self.game_view);
            button.draw(&mut self.window, &mut text);
        }
    }

    pub fn set_view_zoom(&mut self, factor: f32) {
        self.game_view.zoom(factor);
    }

    pub fn reset_zoom(&mut self) {
        self.game_view.set_size(Vector2f::new(
            self.window.size().x as f32,
            self.window.size().y as f32,
        ));
    }

    pub fn save_config(&self) {
        self.config.to_file(format!("{}{}{}", App::CONFIG_PATH, self.config.profile as u8 + 1, App::CONFIG_FILE_EXTENSION).as_str());
    }
}
