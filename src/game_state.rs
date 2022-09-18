use sfml::{
    graphics::{Color, RenderTarget},
    system::Vector2f,
};

use crate::{
    app::App,
    window::{
        ui::{AnswerData, ButtonAction, TextButton, TextDescriptor},
        ViewEnum,
    },
};

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
            Color::WHITE,
            true,
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
            Vector2f::new(app.window.size().x as f32 / 2.0, app.window.size().y as f32 - 100.0),
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::GotoMenu,
            ViewEnum::DefaultView,
        );
        app.buttons.borrow_mut().push(back_button);

        app.kanji_dealer.update_kanji_pool(&app.config);
        let correct_answer = app.kanji_dealer.deal_kanji();

        let mut kanji_text = TextDescriptor::new(
            &correct_answer.kanji.to_string(),
            Vector2f::new(app.window.size().x as f32 / 2.0, 50.0),
            Color::WHITE,
            true,
        );
        kanji_text.font_base_size = 50;
        app.texts.push(kanji_text);

        let mut last_btn_height = 0.0;
        
        let (correct_index, candidates) = app.kanji_dealer.deal_kanji_candidates(correct_answer);
        for (i, option) in candidates.into_iter().enumerate() {
            let button_string: String = if i as u8 == correct_index {
                if app.config.romaji_enabled {
                    correct_answer.as_romaji()
                } else {
                    correct_answer.joyo_reading.clone()
                }
            } else if app.config.romaji_enabled {
                option.as_romaji()
            } else {
                option.joyo_reading.clone()
            };

            let pos = Vector2f::new(app.window.size().x as f32 / 2.0, 200.0 + last_btn_height);
            let button = TextButton::new(
                &button_string,
                pos,
                Color::WHITE,
                Color::WHITE,
                app,
                ButtonAction::CheckAnswer(AnswerData {
                    correct_index: correct_index as u8,
                    index_to_test: i as u8,
                    button_id: TextButton::generate_id_from_pos(pos),
                    kanji: correct_answer.kanji,
                }),
                ViewEnum::GameButtonsView,
            );
            last_btn_height = (i + 1) as f32 * (button.get_height() + 50.0);
            app.buttons.borrow_mut().push(button);
        }
    }

    pub fn init_options_state(app: &mut App) {
        app.texts.clear();
        app.buttons.borrow_mut().clear();
    }
}
