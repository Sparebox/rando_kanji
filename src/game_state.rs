use sfml::{
    graphics::{Color, RenderTarget},
    system::Vector2f,
};

use crate::{
    app::App,
    config::ButtonTextOption,
    window::ui::{AnswerData, ButtonAction, TextButton, TextDescriptor},
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameState {
    Menu,
    Options,
    Play,
}

impl GameState {
    pub const MENU_TITLE: &'static str = "Rando Kanji ・ ランド漢字";

    pub fn init_menu_state(app: &mut App) {
        app.window.set_framerate_limit(App::MENU_FPS_LIMIT);
        app.reset_zoom();
        app.texts.clear();
        app.buttons.borrow_mut().clear();

        let mut title = TextDescriptor::new(
            Self::MENU_TITLE,
            Vector2f::new(app.window.size().x as f32 / 2.0, 100.0),
            Color::WHITE,
            true,
        );
        title.font_size = App::FONT_SIZE;
        app.texts.push(title);
    }

    pub fn init_play_state(app: &mut App) {
        app.window.set_framerate_limit(App::GAME_FPS_LIMIT);
        app.reset_zoom();
        app.texts.clear();
        app.buttons.borrow_mut().clear();

        app.kanji_dealer.update_kanji_pool(&mut app.config);

        let (correct_index, candidates) = app.kanji_dealer.deal_kanji_candidates(&mut app.config);

        let mut kanji_text = TextDescriptor::new(
            &candidates[correct_index as usize].kanji.to_string(),
            Vector2f::new(app.window.size().x as f32 / 2.0, 50.0),
            Color::WHITE,
            true,
        );
        kanji_text.font_size = 50 + App::FONT_SIZE;
        app.texts.push(kanji_text);

        let mut last_btn_height = 0.0;

        for (i, option) in candidates.iter().enumerate() {
            let button_string: String = if i as u8 == correct_index {
                match app.config.button_text_option {
                    ButtonTextOption::Kana => candidates[correct_index as usize]
                        .joyo_reading
                        .trim()
                        .to_string(),
                    ButtonTextOption::Romaji => candidates[correct_index as usize].as_romaji(),
                    ButtonTextOption::Meaning => candidates[correct_index as usize].as_meaning(),
                }
            } else {
                match app.config.button_text_option {
                    ButtonTextOption::Kana => option.joyo_reading.trim().to_string(),
                    ButtonTextOption::Romaji => option.as_romaji(),
                    ButtonTextOption::Meaning => option.as_meaning(),
                }
            };

            let pos = Vector2f::new(app.window.size().x as f32 / 2.0, 200.0 + last_btn_height);
            let mut button = TextButton::new(
                &button_string,
                pos,
                Color::WHITE,
                Color::WHITE,
                &app.font,
                ButtonAction::CheckAnswer(AnswerData {
                    correct_index: correct_index as u8,
                    index_to_test: i as u8,
                    kanji: candidates[correct_index as usize].kanji,
                }),
            );
            button.text.font_size = App::FONT_SIZE;
            last_btn_height = (i + 1) as f32 * (button.get_height() + 50.0);
            app.buttons.borrow_mut().push(button);
        }
    }

    pub fn init_options_state(app: &mut App) {
        app.texts.clear();
        app.buttons.borrow_mut().clear();
    }
}
