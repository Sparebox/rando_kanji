use rand::{seq::{SliceRandom}, Rng};
use sfml::{graphics::{RenderTarget, Color}, system::{Vector2f, Vector2i}, window::Event};

use crate::{window::ui::{TextDescriptor, TextButton, ButtonAction, AnswerData}, App, kanji::KanjiRecord};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GameState {
    Menu,
    Options,
    Play,
}

impl GameState {

    pub fn init_menu_state(app: &mut App) {
        app.texts.clear();
        app.buttons.borrow_mut().clear();
        let mut height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
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
        );
        app.buttons.borrow_mut().push(button);
    }

    pub fn init_play_state(app: &mut App) {
        let mut height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
        app.texts.clear();
        app.buttons.borrow_mut().clear();
        let back_button = TextButton::new(
            "Back",
            Vector2f::new(app.win_size.x / 2.0, app.win_size.y - 100.0),
            Color::WHITE,
            Color::WHITE,
            app,
            ButtonAction::GotoMenu,
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
        let mut y_offset = 0.0;

        for (i, option) in app.kanjis
        .as_slice()
        .choose_multiple(&mut rand::thread_rng(), 4)
        .collect::<Vec<&KanjiRecord>>()
        .into_iter()
        .enumerate() {
            let string = if i == correct_index {
                &kanji_record.joyo_reading
            } else {
                &option.joyo_reading
            };
            let pos = Vector2f::new(app.win_size.x / 2.0, 300.0 + y_offset);
            let button = TextButton::new(
                string,
                pos,
                Color::WHITE,
                Color::WHITE,
                app,
                ButtonAction::CheckAnswer(
                    AnswerData {
                        correct_index: correct_index as u8,
                        index_to_test: i as u8,
                        button_id: TextButton::generate_id_from_pos(pos),
                    }
                ),
            );
            app.buttons.borrow_mut().push(button);
            y_offset += 100.0;
        }
    }
}