use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Kanji {
    pub id: u32,
    pub kanji: char,
    pub jlpt: u8,
    pub joyo_reading: Box<str>,
    pub reading: Box<str>,
    pub on: Box<str>,
    pub on_trans: Box<str>,
    pub kun: Box<str>,
    pub kun_trans: Box<str>,
}

impl Kanji {
    pub fn from_csv(path: &Path) -> Result<Vec<Kanji>, csv::Error> {
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
        rdr.deserialize()
            .collect::<Result<Vec<Kanji>, csv::Error>>()
    }
}
