use std::error::Error;

use chrono::Local;

use crate::calories_data::CaloriesData;

#[derive(Debug)]
enum Command {
    Add,
    List,
    ListDetails,
    Clear,
    Pop,
    Help,
}

pub struct CliHandler {
    calories_data: CaloriesData,
}

impl CliHandler {
    pub fn new(calories_data: CaloriesData) -> Self {
        CliHandler { calories_data }
    }

    fn parse_args(args: Vec<String>) -> (Option<Command>, Option<u32>) {
        match args.get(0).map(String::as_str) {
            Some("add" | "-a") => (Some(Command::Add), args.get(1).and_then(|v| v.parse().ok())),
            Some("ls" | "list" | "-l") => (Some(Command::List), None),
            Some("listdetails" | "-ld") => (Some(Command::ListDetails), None),
            Some("clear" | "-c") => (Some(Command::Clear), None),
            Some("pop" | "-p") => (Some(Command::Pop), None),
            Some("help" | "-h") => (Some(Command::Help), None),
            _ => (None, None),
        }
    }

    pub fn handle_args(&mut self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let (command, value) = Self::parse_args(args);
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();

        match command {
            Some(Command::Add) => {
                if let Some(calories_to_add) = value {
                    let calories = self.calories_data.data.get_calories_by_date_mut(&today);

                    match calories {
                        Some(vals) => {
                            vals.push(calories_to_add);
                            println!("Current calories: {}", vals.iter().sum::<u32>());
                        }
                        None => {
                            self.calories_data
                                .data
                                .set_calories_by_date(&today, vec![calories_to_add]);
                            println!("Current calories: {}", calories_to_add);
                        }
                    };
                    self.calories_data.save()?;
                } else {
                    println!("Add requires number");
                }
            }
            Some(Command::Clear) => {
                self.calories_data.data.clear_calories_by_date(&today);
                self.calories_data.save()?;
                println!("Current calories: 0");
            }
            Some(Command::Help) => {
                println!("Available commands:");
                println!("  add <number> | -a <number> : Add calories");
                println!("  list | -l : List total calories for today");
                println!("  listdetails | -ld : List detailed calories entries for today");
                println!("  clear | -c : Clear all calorie entries for today");
                println!("  pop | -p : Remove the last calorie entry for today");
                println!("  help | -h : Display this help message");
            }
            Some(Command::List) => {
                let calories = self.calories_data.data.get_calories_by_date(&today);
                match calories {
                    Some(vals) => println!("Current calories: {}", vals.iter().sum::<u32>()),
                    None => println!("Current calories: 0"),
                };
            }
            Some(Command::ListDetails) => {
                let calories = self.calories_data.data.get_calories_by_date(&today);
                match calories {
                    Some(vals) => {
                        println!("Current calories entries: {:?}", vals);
                        println!("Current calories: {}", vals.iter().sum::<u32>());
                    }
                    None => {
                        println!("Current calories entries: []");
                        println!("Current calories: 0");
                    }
                };
            }
            Some(Command::Pop) => {
                let calories = self.calories_data.data.get_calories_by_date_mut(&today);

                match calories {
                    Some(vals) => {
                        vals.pop();
                        println!("Current calories: {}", vals.iter().sum::<u32>());
                    }
                    None => println!("Current calories: 0"),
                };
                self.calories_data.save()?;
            }
            None => println!("Unknown command. Use 'help' or '-h' for a list of commands"),
        };

        Ok(())
    }
}
