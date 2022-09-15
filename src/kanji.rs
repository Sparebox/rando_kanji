use std::path::Path;

use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;

use crate::app::App;

#[derive(Deserialize, Debug)]
pub struct KanjiRecord {
    pub id: u32,
    pub kanji: char,
    pub jlpt: u8,
    pub joyo_reading: String,
    pub reading: String,
    pub on: String,
    pub on_trans: String,
    pub kun: String,
    pub kun_trans: String,
}

impl KanjiRecord {
    pub fn from_csv(path: &Path) -> Result<Vec<KanjiRecord>, csv::Error> {
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
        rdr.deserialize()
            .collect::<Result<Vec<KanjiRecord>, csv::Error>>()
    }

    pub fn as_romaji(&self) -> String {
        self.on.clone() + " " + self.kun.as_str()
    }
}

impl PartialEq for KanjiRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct KanjiDealer {
    pub kanjis: Vec<KanjiRecord>
}

impl KanjiDealer {
    pub fn new() -> Self {
        let kanjis =
            KanjiRecord::from_csv(Path::new(App::KANJI_DB_PATH)).expect("Could not load kanjis");
        Self {
            kanjis
        }
    }

    pub fn deal_kanji(&self) -> &KanjiRecord {
            self.kanjis
            .as_slice()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    pub fn deal_kanji_candidates<'a>(&'a self, correct_answer: &'a KanjiRecord) -> (u8, Vec<&'a KanjiRecord>) {
        let correct_index: usize = rand::thread_rng().gen_range(0..=3);
        let mut candidates = self.kanjis
            .as_slice()
            .choose_multiple(&mut rand::thread_rng(), 4)
            .collect::<Vec<&KanjiRecord>>();
    
        for record in candidates.as_mut_slice() {
            while *record == correct_answer {
                *record = self.kanjis.as_slice().choose(&mut rand::thread_rng()).unwrap()
            }
        }
        candidates.sort_unstable_by_key(|k| k.id);
        candidates.dedup_by_key(|k| k.id);
        candidates[correct_index] = correct_answer;
        (correct_index as u8, candidates)
    }
}
