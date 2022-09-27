use rand::{Rng, distributions::Uniform, seq::IteratorRandom};
use sfml::{system::Vector2f, graphics::Color};

use crate::{window::ui::TextDescriptor, app::App, kanji::KanjiRecord, game_state::GameState, utils::Timer};

pub struct KanjiFall {
    spawn_timer: Timer,
    columns: [Column; Self::COLUMNS_NUM as usize],
}

impl KanjiFall {
    const COLUMN_WIDTH: u32 = 50;
    const COLUMNS_NUM: u32 = App::INIT_WIN_SIZE.x / Self::COLUMN_WIDTH as u32;
    const MAX_MOVEMENT_SPEED: f32 = 5.0;
    const MAX_TAIL_LENGHT: u8 = 50;
    const MIN_TAIL_LENGHT: u8 = 20;
    const TAIL_MARGIN: Vector2f = Vector2f::new(0.0, 20.0);
    const DROP_INTERVAL_SECS: f32 = 0.1;
    const KANJI_BASE_COLOR: Color = Color::rgba(0, 255, 0, 128);
    const KANJI_FONT_SIZE: u32 = 20;
    const STARTING_Y: f32 = 100.0;
    
    pub fn new() -> Self {
        Self {
            spawn_timer: Timer::new(Self::DROP_INTERVAL_SECS),
            columns: [Column::default(); Self::COLUMNS_NUM as usize],
        }
    }

    pub fn add_to_fall(&mut self, texts: &mut Vec<TextDescriptor>, kanji: &[KanjiRecord]) {
        if !self.spawn_timer.check() {
            return;
        }
        let random_column = rand::thread_rng().sample(Uniform::from(0..Self::COLUMNS_NUM));

        if self.columns[random_column as usize].is_in_use {
            return;
        } else {
            self.columns[random_column as usize].is_in_use = true;
        }
        
        let random_pos = Vector2f::new((random_column * Self::COLUMN_WIDTH) as f32, Self::STARTING_Y);
        
        texts.extend(Self::create_tail(random_pos, Self::KANJI_BASE_COLOR, kanji));
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
                text.color = Color::WHITE;
            }
            
            if let Some(diff) = text.color.r.checked_sub(3) {
                text.color.r = diff;
            }
            if let Some(diff) = text.color.b.checked_sub(3) {
                text.color.b = diff;
            }
            if let Some(diff) = text.color.a.checked_sub(3) {
                text.color.a = diff;
            } else {
                text.color.a = 0;
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
        let tail_length = rand::thread_rng().sample(Uniform::from(Self::MIN_TAIL_LENGHT..Self::MAX_TAIL_LENGHT));
        let mut random_kanji: String;
        let mut text: TextDescriptor;
        for i in 0..=tail_length {
            random_kanji = kanji.iter().choose(&mut rand::thread_rng()).unwrap().kanji.to_string();
            text = TextDescriptor::new(&random_kanji, head_pos, color, true);
            text.font_size = Self::KANJI_FONT_SIZE;
            text.timer.set_duration((tail_length - i) as f32 / 2.0 + 1.0);
            tail.push(text);
            head_pos -= Self::TAIL_MARGIN;
        }
        tail
    }
}

#[derive(Clone, Copy)]
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
