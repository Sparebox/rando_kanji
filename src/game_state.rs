use rand::{seq::{SliceRandom}, Rng};
use sfml::{graphics::{RenderTarget, Color}, system::{Vector2f}};

use crate::{window::{ui::{TextDescriptor, TextButton, ButtonAction, AnswerData}, ViewEnum}, app::App, kanji::KanjiRecord};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GameState {
    Menu,
    Options,
    Play,
}

impl GameState {

    pub fn init_menu_state(app: &mut App) {
        app.reset_zoom();
        app.texts.clear();
        app.buttons.borrow_mut().clear();
        let mut height_offset: f32 = App::FONT_SIZE as f32 * (5.0 / 3.0);
        let mut title = TextDescriptor::new(
            "Rando Kanji ・ ランド漢字",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset),
            Color::WHITE, true
        );
        title.font_base_size = 10;
        app.texts.push(title);
    
        height_offset += 200.0;

        let mut button = TextButton::new(
            "Play",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset), 
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::GotoGame,
            ViewEnum::DefaultView,
        );
        app.buttons.borrow_mut().push(button.clone());
        
        height_offset += 200.0;

        button = TextButton::new(
            "Options",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset),
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::GotoOptions,
            ViewEnum::DefaultView,
        );
        app.buttons.borrow_mut().push(button.clone());

        height_offset += 200.0;

        button = TextButton::new(
            "Exit",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset),
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::ExitGame,
            ViewEnum::DefaultView,
        );
        app.buttons.borrow_mut().push(button);
    }

    pub fn init_play_state(app: &mut App) {
        // let height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
        app.reset_zoom();
        app.texts.clear();
        app.buttons.borrow_mut().clear();
        let back_button = TextButton::new(
            "Back",
            Vector2f::new(app.win_size.x / 2.0, app.win_size.y - 100.0),
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::GotoMenu,
            ViewEnum::DefaultView,
        );
        app.buttons.borrow_mut().push(back_button);

        let kanji_record = app.kanjis
            .as_slice()
            .choose(&mut rand::thread_rng())
            .unwrap();

        let mut kanji_text = TextDescriptor::new(
            &kanji_record.kanji.to_string(),
            Vector2f::new(app.win_size.x / 2.0, 50.0),
            Color::WHITE,
            true,
        );
        kanji_text.font_base_size = 50;
        app.texts.push(kanji_text);

        let correct_index: usize = rand::thread_rng().gen_range(0..=3);
        let mut last_btn_height = 0.0;

        for (i, option) in app.kanjis
        .as_slice()
        .choose_multiple(&mut rand::thread_rng(), 4)
        .collect::<Vec<&KanjiRecord>>()
        .into_iter()
        .enumerate() {

            let button_string: String = if i == correct_index {
                if app.config.romaji_enabled {
                    kanji_record.as_romaji()
                } else {
                    kanji_record.joyo_reading.clone()
                }
            } else if app.config.romaji_enabled {
                option.as_romaji()
            } else {
                option.joyo_reading.clone()
            };
            let pos = Vector2f::new(app.win_size.x / 2.0, 200.0 + last_btn_height);
            let button = TextButton::new(
                &button_string,
                pos,
                Color::WHITE,
                Color::WHITE,
                app,
                ButtonAction::CheckAnswer(
                    AnswerData {
                        correct_index: correct_index as u8,
                        index_to_test: i as u8,
                        button_id: TextButton::generate_id_from_pos(pos),
                        kanji: kanji_record.kanji,
                    }
                ),
                ViewEnum::GameButtonsView,
            );
            last_btn_height = (i + 1) as f32 * (button.get_height() + 50.0);
            app.buttons.borrow_mut().push(button);
        }
    }

    pub fn init_options_state(app: &mut App) {
        app.texts.clear();
        app.buttons.borrow_mut().clear();

        // let mut romaji_button = TextButton::new(
        //     "Toggle Rōmaji ローマ字",
        //     Vector2f::new(app.win_size.x / 4.0, 100.0),
        //     Color::WHITE,
        //     Color::WHITE,
        //     app,
        //     ButtonAction::ToggleRomaji,
        //     ViewEnum::DefaultView,
        // );

        // if app.config.romaji_enabled {
        //     romaji_button.set_color(Color::GREEN, true);
        // }

        // app.buttons.borrow_mut().push(romaji_button);
        
        // let reset_config_button = TextButton::new(
        //     "Reset configurations",
        //     Vector2f::new(3.0 * app.win_size.x / 4.0, 100.0),
        //     Color::WHITE,
        //     Color::WHITE,
        //     app,
        //     ButtonAction::ResetConfig,
        //     ViewEnum::DefaultView,
        // );

        // app.buttons.borrow_mut().push(reset_config_button);

        // let back_button = TextButton::new(
        //     "Back",
        //     Vector2f::new(app.win_size.x / 2.0, app.win_size.y - 100.0),
        //     Color::WHITE,
        //     Color::WHITE,
        //     app,
        //     ButtonAction::GotoMenu,
        //     ViewEnum::DefaultView,
        // );
        // app.buttons.borrow_mut().push(back_button);
    }
}