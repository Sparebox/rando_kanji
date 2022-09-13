use std::{path::Path, fs::File, io::{BufReader, BufWriter}, error::Error};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub romaji_enabled: bool
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
        let file = File::create(path)
            .expect("Could not save config file");
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
    }
}