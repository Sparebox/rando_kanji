use rand::{seq::{SliceRandom}, Rng};
use sfml::{graphics::{RenderTarget, Color}, system::{Vector2f, Vector2i}, window::Event};

use crate::{window::ui::{TextDescriptor, TextButton}, App, kanji::KanjiRecord};

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
            |app| app.change_state(GameState::Play),
        );
        app.buttons.borrow_mut().push(button.clone());
        
        height_offset += 200.0;

        button = TextButton::new(
            "Options",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset),
            Color::WHITE,
            Color::WHITE,
            app,
            |app| {},
        );
        app.buttons.borrow_mut().push(button.clone());

        height_offset += 200.0;

        button = TextButton::new(
            "Exit",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset),
            Color::WHITE,
            Color::WHITE,
            app,
            |app| app.window.close(),
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
            |app| { app.change_state(GameState::Menu) },
        );
        app.buttons.borrow_mut().push(back_button);

        let new_button = TextButton::new(
            "New",
            Vector2f::new(app.win_size.x / 2.0, app.win_size.y - 200.0),
            Color::WHITE,
            Color::WHITE,
            app,
            |app| { app.change_state(GameState::Play) },
        );
        app.buttons.borrow_mut().push(new_button);

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

        height_offset += 50.0;

        let joyo_reading = TextDescriptor::new(
            &("Kana: ".to_string() + &kanji_record.joyo_reading),
            Vector2f::new(50.0, height_offset),
            Color::WHITE,
            false,
        );
        app.texts.push(joyo_reading);

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
            let button = TextButton::new(
                string,
                Vector2f::new(app.win_size.x / 2.0, 200.0 + y_offset),
                Color::WHITE,
                Color::WHITE,
                app,
                |app| {},
            );
            app.buttons.borrow_mut().push(button);
            y_offset += 200.0;
        }

    }

    pub fn handle_events(app: &mut App, event: &Event) {
        match event {
            Event::MouseButtonPressed { button: _, x, y } => 
                Self::update_buttons(app, Vector2i::new(*x, *y), true),
            Event::MouseMoved { x, y } => 
                Self::update_buttons(app, Vector2i::new(*x, *y), false),
            _ => {/* Do nothing */},
        }
    }

    pub fn update_buttons(app: &mut App, mouse_pos: Vector2i, check_press: bool) {
        for button in app.buttons.clone().borrow_mut().iter_mut() {
            if check_press {
                button.check_mouse_press(mouse_pos, app);
            } else {
                button.check_mouse_hover(mouse_pos);
            }
        }
    }

}