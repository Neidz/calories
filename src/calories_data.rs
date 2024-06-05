use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Date(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Data(HashMap<Date, Vec<u32>>);

impl Data {
    pub fn get_calories_by_date(&self, date: &str) -> Option<&Vec<u32>> {
        let hashmap = &self.0;

        hashmap.get(&Date(date.to_string()))
    }

    pub fn get_calories_by_date_mut(&mut self, date: &str) -> Option<&mut Vec<u32>> {
        let hashmap = &mut self.0;

        hashmap.get_mut(&Date(date.to_string()))
    }

    pub fn set_calories_by_date(&mut self, date: &str, calories: Vec<u32>) {
        let hashmap = &mut self.0;

        hashmap.insert(Date(date.to_string()), calories);
    }

    pub fn clear_calories_by_date(&mut self, date: &str) {
        let hashmap = &mut self.0;

        hashmap.remove(&Date(date.to_string()));
    }
}

#[derive(Debug)]
pub struct CaloriesData {
    pub data: Data,
    path: PathBuf,
}

impl CaloriesData {
    pub fn new(path: PathBuf) -> Result<CaloriesData, Box<dyn Error>> {
        let data = Self::load(&path)?;

        Ok(CaloriesData { path, data })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self.data)?;
        let mut file = File::create(&self.path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load(path: &Path) -> Result<Data, Box<dyn Error>> {
        let file = if path.exists() {
            File::open(&path)?
        } else {
            Self::create_default(&path)?;
            File::open(&path)?
        };

        let reader = BufReader::new(file);
        let data: Data = serde_json::from_reader(reader)?;

        Ok(data)
    }

    fn create_default(path: &Path) -> Result<(), Box<dyn Error>> {
        let parent_dir = path.parent().ok_or("Unable to find parent directory")?;
        fs::create_dir_all(parent_dir)?;

        let default_data: Data = Data(HashMap::new());
        let default_json = serde_json::to_string_pretty(&default_data)?;

        let mut new_file = fs::File::create(path)?;
        new_file.write_all(default_json.as_bytes())?;

        println!(
            "Calories data not found, created new calories data at: {:?}",
            path
        );
        Ok(())
    }
}
