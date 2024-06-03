use calories_data::CaloriesData;
use config::Config;

mod calories_data;
mod config;

fn main() {
    let config = Config::load().expect("Config not loaded");
    let calories_data = CaloriesData::load(&config.data_path).expect("Data not loaded");

    println!("{:?}", calories_data);
}
