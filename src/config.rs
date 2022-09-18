use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub romaji_enabled: bool,
    pub learned_threshold: i32, // Amount of correct answers for a kanji to be considered learned
    pub kanji_pool_max_size: u32, // Size of the kanji pool at the start of a new pool cycle
    pub answer_statistics: HashMap<char, i32>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn to_file(&self, path: &str) {
        let path = Path::new(path);
        match File::create(path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                let _ = serde_json::to_writer(writer, self);
            }
            Err(err) => eprintln!("{}", err),
        };
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            romaji_enabled: false,
            learned_threshold: 5,
            kanji_pool_max_size: 10,
            answer_statistics: HashMap::default(),
        }
    }
}
