use std::path::Path;

use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;

use crate::{app::App, config::Config};

#[derive(Deserialize, Debug)]
pub struct KanjiRecord {
    pub id: u32,
    pub kanji: char,
    pub jlpt: u8,
    pub joyo_reading: String,
    pub reading: String,
    #[serde(rename = "on")]
    pub on_reading: String,
    pub on_trans: String,
    #[serde(rename = "kun")]
    pub kun_reading: String,
    pub kun_trans: String,
}

impl KanjiRecord {
    pub fn from_csv(path: &Path) -> Result<Vec<KanjiRecord>, csv::Error> {
        let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
        reader.deserialize()
            .collect::<Result<Vec<KanjiRecord>, csv::Error>>()
    }

    pub fn as_romaji(&self) -> String {
        self.on_reading.clone() + " " + self.kun_reading.as_str()
    }
}

impl PartialEq for KanjiRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct KanjiDealer {
    pub kanjis: Vec<KanjiRecord>,
    pub kanji_pool: Vec<char>, // Vector of kanji chars
    pub last_dealt_kanji_id: u32,
}

impl KanjiDealer {
    pub fn new() -> Self {
        let kanjis =
            KanjiRecord::from_csv(Path::new(App::KANJI_DB_PATH)).expect("Could not load kanjis");
        Self {
            kanjis,
            kanji_pool: Vec::<char>::new(),
            last_dealt_kanji_id: 0,
        }
    }
    /// Add kanji to the pool for spaced learning
    /// based on the learning threshold set in Config.
    /// i.e ignore the kanji that have been guessed right enough times
    pub fn update_kanji_pool(&mut self, config: &Config) {
        if self.kanji_pool.is_empty() {
            if config.answer_statistics.is_empty() { // If no previous data is available
                self.add_new_kanji_to_pool(config);
            } else { // Load previous statistics to kanji pool
                for entry in config.answer_statistics.iter() {
                    if self.kanji_pool.len() as u32 == config.kanji_pool_max_size {
                        break;
                    }
                    self.kanji_pool.push(*entry.0);
                }
                self.minimize_kanji_pool(config);
            }

        } else {
            self.minimize_kanji_pool(config);
        }
    }

    pub fn deal_kanji(&self) -> &KanjiRecord {
            let pool_char = self.kanji_pool
                .as_slice()
                .choose(&mut rand::thread_rng())
                .expect("Kanji pool was empty for some reason");
            
            self.kanjis
                .iter()
                .find(|record| record.kanji == *pool_char)
                .expect("Could not find record from kanji pool")
    }

    pub fn deal_kanji_candidates<'a>(&'a self, correct_answer: &'a KanjiRecord) -> (u8, Vec<&'a KanjiRecord>) {
        let correct_index: usize = rand::thread_rng().gen_range(0..=3);
        let mut candidates = Vec::<&KanjiRecord>::new();
        if self.kanji_pool.len() > 1 {
            let char_candidates = self.kanji_pool
                .as_slice()
                .choose_multiple(&mut rand::thread_rng(), 4)
                .collect::<Vec<&char>>();

            char_candidates
                .iter()
                .for_each(|char| candidates.push(self.find_record_by_char(char)));

            if char_candidates.len() < 4 {
                let to_add = 4 - char_candidates.len();
                let mut kanji_to_add = self.kanjis
                    .as_slice()
                    .choose_multiple(&mut rand::thread_rng(), to_add)
                    .collect::<Vec<&KanjiRecord>>();
                candidates.append(&mut kanji_to_add);
            }

        } else {
            candidates = self.kanjis
                .as_slice()
                .choose_multiple(&mut rand::thread_rng(), 4)
                .collect::<Vec<&KanjiRecord>>();
        }
        // Remove possible duplicate correct answers
        for record in candidates.as_mut_slice() {
            while *record == correct_answer {
                *record = self.kanjis
                    .as_slice()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
            }
        }
        // Remove possible duplicate kanji reading options
        candidates.sort_unstable_by_key(|k| k.id);
        candidates.dedup_by_key(|k| k.id);
        // Add correct answer option
        candidates[correct_index] = correct_answer;
        (correct_index as u8, candidates)
    }

    fn minimize_kanji_pool(&mut self, config: &Config) {
        for entry in config.answer_statistics.iter() {
            if *entry.1 >= config.learned_threshold {
                self.kanji_pool.retain(|kanji| *kanji != *entry.0);
                if self.kanji_pool.is_empty() {
                    break;
                }
            }
        }
        if self.kanji_pool.is_empty() {
            self.add_new_kanji_to_pool(config);
        }
    }

    fn add_new_kanji_to_pool(&mut self, config: &Config) {
        self.kanjis.shuffle(&mut rand::thread_rng());
        for record in self.kanjis.as_slice() {
            if self.kanji_pool.len() as u32 == config.kanji_pool_max_size {
                break;
            } else if !config.answer_statistics.contains_key(&record.kanji) {
                self.kanji_pool.push(record.kanji);
            }
        }
    }

    fn find_record_by_char(&self, char: &char) -> &KanjiRecord {
        self.kanjis.iter()
            .find(|record| record.kanji == *char)
            .expect("Could not find record by char")
    }

}
