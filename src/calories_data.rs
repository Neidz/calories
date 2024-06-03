use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct Date(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct CaloriesData(HashMap<Date, Vec<i32>>);

impl CaloriesData {
    pub fn load(path: &Path) -> Result<CaloriesData, Box<dyn Error>> {
        let file = if path.exists() {
            File::open(&path)?
        } else {
            Self::create_default(&path)?;
            File::open(&path)?
        };

        let reader = BufReader::new(file);
        let calories_data: CaloriesData = serde_json::from_reader(reader)?;

        Ok(calories_data)
    }

    fn create_default(path: &Path) -> Result<(), Box<dyn Error>> {
        let parent_dir = path.parent().ok_or("Unable to find parent directory")?;
        fs::create_dir_all(parent_dir)?;

        let default_data: CaloriesData = CaloriesData(HashMap::new());
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
