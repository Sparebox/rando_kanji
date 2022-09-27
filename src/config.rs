use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    time::{Duration, SystemTime}, fmt::Display,
};

use serde::{Deserialize, Serialize};

use crate::{app::App};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub profile: Profile,
    pub was_used_last: bool,
    pub button_text_option: ButtonTextOption,
    pub learning_index_threshold: i32, // Value of learning index for a kanji to be considered learned
    pub kanji_pool_max_size: u32,      // Size of the kanji pool at the start of a new pool cycle
    pub answer_statistics: HashMap<char, StatValue>,
}

impl Config {
    pub const REVIEW_INTERVAL_STEP: Duration = Duration::from_secs(Self::minutes_to_seconds(5)); // Time step to increase time between review intervals

    fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    fn to_file(&self, path: &str) {
        let path = Path::new(path);
        match File::create(path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self).expect("Could not save config to disk!");
            }
            Err(err) => eprintln!("Error saving configuration: {}", err),
        };
    }

    fn get_filename(profile: ProfileEnum) -> String {
        match profile {
            ProfileEnum::Profile1 => format!("{}{}{}", App::CONFIG_PATH, 1, App::CONFIG_FILE_EXTENSION),
            ProfileEnum::Profile2 => format!("{}{}{}", App::CONFIG_PATH, 2, App::CONFIG_FILE_EXTENSION),
            ProfileEnum::Profile3 => format!("{}{}{}", App::CONFIG_PATH, 3, App::CONFIG_FILE_EXTENSION),
        }
    }

    #[inline]
    fn filename(&self) -> String {
        Self::get_filename(self.profile.id)
    }

    #[inline]
    pub fn save(&self) {
        self.to_file(&self.filename());
    }

    pub fn load_from_file() -> Self {
        let mut config_to_return : Option<Config> = None;
        for i in 1..=3 {
            if let Ok(mut config) = Self::from_file(format!("{}{}{}", App::CONFIG_PATH, i, App::CONFIG_FILE_EXTENSION).as_str()) {
                if config.was_used_last {
                    config.reset_review_times();
                    return config;
                } else { // In case there was no last used profile detected for some reason
                    config_to_return = Some(config);
                }
            }
        }
        if let Some(mut config) = config_to_return {
            config.reset_review_times();
            config
        } else {
            eprintln!("Could not load config from file");
            Self::default()
        }
    }

    #[inline]
    pub fn try_load_by_profile(profile: ProfileEnum) -> Result<Config, Box<dyn Error>> {
        let filename = Self::get_filename(profile);
        Self::from_file(&filename)
    }

    pub fn reset_review_times(&mut self) {
        self.answer_statistics
            .iter_mut()
            .for_each(|stat| stat.1.last_review_time = SystemTime::now())
    }

    pub fn reset_last_used() {
        for profile in [ProfileEnum::Profile1, ProfileEnum::Profile2, ProfileEnum::Profile3] {
            if let Ok(mut loaded_profile) = Self::try_load_by_profile(profile) {
                loaded_profile.was_used_last = false;
                loaded_profile.save();
            }
        }
    }

    // const fn days_to_seconds(days: u64) -> u64 {
    //     days * 24 * 60 * 60
    // }
    #[inline]
    const fn minutes_to_seconds(minutes: u64) -> u64 {
        minutes * 60
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: Profile::default(),
            was_used_last: false,
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

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub id: ProfileEnum,
}

impl Default for Profile {
    fn default() -> Self {
        Self { 
            name: "Default profile".to_string(),
            id: ProfileEnum::Profile1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProfileEnum {
    Profile1,
    Profile2,
    Profile3,
}

impl Display for ProfileEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileEnum::Profile1 => write!(f, "Profile 1"),
            ProfileEnum::Profile2 => write!(f, "Profile 2"),
            ProfileEnum::Profile3 => write!(f, "Profile 3"),
        }
        
    }
}
