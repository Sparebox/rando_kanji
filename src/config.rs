use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    time::{Duration, SystemTime},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub profile: ConfigProfile,
    pub button_text_option: ButtonTextOption,
    pub learning_index_threshold: i32, // Value of learning index for a kanji to be considered learned
    pub kanji_pool_max_size: u32,      // Size of the kanji pool at the start of a new pool cycle
    pub answer_statistics: HashMap<char, StatValue>,
}

impl Config {
    pub const REVIEW_INTERVAL_STEP: Duration = Duration::from_secs(Self::minutes_to_seconds(5)); // Time step to increase time between review intervals

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
                serde_json::to_writer_pretty(writer, self).expect("Could not save config to disk!");
            }
            Err(err) => eprintln!("Error saving configuration: {}", err),
        };
    }

    pub fn reset_review_times(&mut self) {
        self.answer_statistics
            .iter_mut()
            .for_each(|stat| stat.1.last_review_time = SystemTime::now())
    }

    // const fn days_to_seconds(days: u64) -> u64 {
    //     days * 24 * 60 * 60
    // }

    const fn minutes_to_seconds(minutes: u64) -> u64 {
        minutes * 60
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: ConfigProfile::Profile1, 
            button_text_option: ButtonTextOption::Kana,
            learning_index_threshold: 5,
            kanji_pool_max_size: 10,
            answer_statistics: HashMap::with_capacity(10),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StatValue {
    pub learning_index: i32,
    pub last_review_time: SystemTime,
    pub review_interval: Duration,
}

impl Default for StatValue {
    fn default() -> Self {
        StatValue {
            learning_index: 0,
            last_review_time: SystemTime::now(),
            review_interval: Duration::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum ButtonTextOption {
    Kana,
    Romaji,
    Meaning,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum ConfigProfile {
    Profile1,
    Profile2,
    Profile3,
}
