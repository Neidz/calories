use std::env;

use calories_data::CaloriesData;
use cli_handler::CliHandler;
use config::Config;

mod calories_data;
mod cli_handler;
mod config;

fn main() {
    let config = Config::load().expect("Failed to load configuration file");
    let calories_data = CaloriesData::new(config.data_path).expect("Failed to load calories data");
    let mut cli_handler = CliHandler::new(calories_data);

    let args: Vec<String> = env::args().skip(1).collect();
    match cli_handler.handle_args(args) {
        Ok(_) => {}
        Err(err) => println!("Something went wrong: {}", err),
    }
}
