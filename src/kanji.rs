use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Kanji {
    id: u32,
    kanji: char,
    jlpt: u8,
    joyo_reading: String,
    reading: String,
    on: String,
    kun: String,
    on_trans: String,
    kun_trans: String,
}

impl Kanji {
    pub fn from_csv(path: &Path) -> Result<Vec<Kanji>, csv::Error> {
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
        rdr.deserialize()
            .collect::<Result<Vec<Kanji>, csv::Error>>()
    }
}
