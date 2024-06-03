use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn Error>> {
        let config_path = Self::get_config_path()?;

        let file = if config_path.exists() {
            File::open(&config_path)?
        } else {
            Self::create_default(&config_path)?;
            File::open(&config_path)?
        };

        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;

        Ok(config)
    }

    fn create_default(config_path: &Path) -> Result<(), Box<dyn Error>> {
        let parent_dir = config_path
            .parent()
            .ok_or("Unable to find parent directory")?;
        fs::create_dir_all(parent_dir)?;

        let default_data_path = Self::get_default_data_path()?;
        let default_config = Config {
            data_path: default_data_path,
        };
        let default_config_json = serde_json::to_string_pretty(&default_config)?;

        let mut new_file = fs::File::create(config_path)?;
        new_file.write_all(default_config_json.as_bytes())?;

        println!("Config not found, created new config at: {:?}", config_path);
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
        let home_dir = env::var("HOME")?;

        let mut path = PathBuf::from(home_dir);
        path.push(".config");
        path.push("calories");
        path.push("config.json");

        Ok(path)
    }

    fn get_default_data_path() -> Result<PathBuf, Box<dyn Error>> {
        let home_dir = env::var("HOME")?;

        let mut path = PathBuf::from(home_dir);
        path.push(".local");
        path.push("share");
        path.push("calories");
        path.push("data.json");

        Ok(path)
    }
}
