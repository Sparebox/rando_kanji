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
    pub const MENU_BTN_POS: Vector2f = Vector2f::new(App::INIT_WIN_SIZE.x as f32 / 2.0, App::INIT_WIN_SIZE.y as f32 - 100.0);
    
    pub fn init_menu_state(app: &mut App) {
        app.reset_zoom();
        app.texts.clear();
        app.buttons.borrow_mut().clear();
        
        let mut title = TextDescriptor::new(
            "Rando Kanji ・ ランド漢字",
            Vector2f::new(app.window.size().x as f32 / 2.0, 100.0),
            Color::WHITE,
            true,
        );
        title.font_base_size = 10;
        app.texts.push(title);
    }

    pub fn init_play_state(app: &mut App) {
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
        kanji_text.font_base_size = 50;
        app.texts.push(kanji_text);

        let mut last_btn_height = 0.0;
        
        for (i, option) in candidates.iter().enumerate() {
            let button_string: String = if i as u8 == correct_index {
                if app.config.romaji_enabled {
                    candidates[correct_index as usize].as_romaji()
                } else if app.config.show_meaning_enabled {
                    candidates[correct_index as usize].as_meaning()
                } else {
                    candidates[correct_index as usize].joyo_reading.trim().to_string()
                }
            } else if app.config.romaji_enabled {
                option.as_romaji()
            } else if app.config.show_meaning_enabled {
                option.as_meaning()
            } else {
                option.joyo_reading.trim().to_string()
            };

            let pos = Vector2f::new(app.window.size().x as f32 / 2.0, 200.0 + last_btn_height);
            let button = TextButton::new(
                &button_string,
                pos,
                Color::WHITE,
                Color::WHITE,
                &app.font,
                ButtonAction::CheckAnswer(AnswerData {
                    correct_index: correct_index as u8,
                    index_to_test: i as u8,
                    button_id: TextButton::generate_id_from_pos(pos),
                    kanji: candidates[correct_index as usize].kanji,
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
