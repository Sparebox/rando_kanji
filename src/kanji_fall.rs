use rand::{Rng, distributions::Uniform, seq::IteratorRandom};
use sfml::{system::Vector2f, graphics::Color};

use crate::{window::ui::TextDescriptor, app::App, kanji::KanjiRecord, game_state::GameState, utils::Timer};

pub struct KanjiFall {
    spawn_timer: Timer,
    columns: Vec<Column>,
}

impl KanjiFall {
    const COLUMN_WIDTH: u32 = 50;
    const COLUMNS_NUM: u32 = App::INIT_WIN_SIZE.x / Self::COLUMN_WIDTH as u32;
    const MAX_MOVEMENT_SPEED: f32 = 5.0;
    const MAX_NUMBER_OF_KANJI: u8 = 20;
    const MAX_TAIL_LENGHT: u8 = 5;
    const TAIL_MARGIN: Vector2f = Vector2f::new(0.0, 50.0);
    const DROP_INTERVAL_SECS: f32 = 0.5;
    const BLINK_INTERVAL_SECS: f32 = 1.0;
    
    pub fn new() -> Self {
        Self {
            spawn_timer: Timer::new(Self::DROP_INTERVAL_SECS),
            columns: vec![Column::default(); Self::COLUMNS_NUM as usize],
        }
    }

    pub fn add_to_fall(&mut self, texts: &mut Vec<TextDescriptor>, kanji: &[KanjiRecord]) {
        if texts.len() == Self::MAX_NUMBER_OF_KANJI as usize || !self.spawn_timer.check() {
            return;
        }
        let random_column = rand::thread_rng().sample(Uniform::from(0..Self::COLUMNS_NUM));

        if self.columns[random_column as usize].is_in_use {
            return;
        } else {
            self.columns[random_column as usize].is_in_use = true;
        }
        
        let random_pos = Vector2f::new((random_column * Self::COLUMN_WIDTH) as f32, -30.0);
        let color = Color::rgba(0, 255, 0, 128);
        
        texts.extend(Self::create_tail(random_pos, color, kanji));
    }

    pub fn update(&mut self, texts: &mut Vec<TextDescriptor>, kanji: &[KanjiRecord]) {
        texts.retain(|text| text.pos.y < App::INIT_WIN_SIZE.y as f32);
        for text in texts.iter_mut() {
            if text.string == GameState::MENU_TITLE {
                continue;
            }
            text.pos.y += Self::MAX_MOVEMENT_SPEED;
            if text.timer.check() {
                text.string = kanji
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .kanji
                    .to_string();
            }
        }
        
        for column in self.columns.iter_mut() {
            if column.timer.check() {
                column.is_in_use = false;
            }
        }

        self.add_to_fall(texts, kanji);
    }

    fn create_tail(mut head_pos: Vector2f, color: Color, kanji: &[KanjiRecord]) -> Vec<TextDescriptor> {
        let mut tail: Vec<TextDescriptor> = Vec::new();
        let tail_length = rand::thread_rng().sample(Uniform::from(0..Self::MAX_TAIL_LENGHT));
        let mut random_kanji: String;
        let mut text: TextDescriptor;
        // tail.push(head_text);
        for i in 0..=tail_length {
            random_kanji = kanji.iter().choose(&mut rand::thread_rng()).unwrap().kanji.to_string();
            text = TextDescriptor::new(&random_kanji, head_pos, color, true);
            text.timer.set_duration(Self::BLINK_INTERVAL_SECS + i as f32);
            tail.push(text);
            head_pos -= Self::TAIL_MARGIN;
        }
        tail
    }
}

#[derive(Clone)]
struct Column {
    is_in_use: bool,
    timer: Timer,
}

impl Column {
    const COLUMN_INTERVAL_SECS: f32 = 10.0;
}

impl Default for Column {
    fn default() -> Self {
        Self {
            is_in_use: false,
            timer: Timer::new(Self::COLUMN_INTERVAL_SECS),
        }
    }
}