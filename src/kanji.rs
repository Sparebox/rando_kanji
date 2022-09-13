use std::path::Path;

use serde::Deserialize;

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
