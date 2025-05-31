use std::fs::File;
use std::io::{self, Write};

pub mod task;
use task::Task;

pub struct Config {
    pub file: File,
    pub list: Vec<Task>,
}

impl Config {
    pub fn build(path: &str) -> Config {
        let file = File::open(path).unwrap_or_else(|_| {
            File::create(path).expect("Couldn't create a file")
        });

        let list: Vec<Task> = Vec::new();

        Config {
            file,
            list,
        }
    }
}

pub fn get_input(msg: &str) -> Result<String, io::Error> {
    print!("{}", msg);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;

    Ok(input)
}

pub fn validate_name(name: String) {
    // If name doesn't exist for another task
}

pub fn validate_date(date: String) {
    // Validate format
}

