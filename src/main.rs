mod task;

use std::{fs, env};
use std::io::{self, BufRead, BufReader, Write};

use clap::{Arg, Command, command};

use task::Task;


fn main() {
    let matches = command!()
        .subcommand(
    Command::new("add")
                .about("Add a new task")
                /*
                .arg(
                    Arg::new("name")
                        .index(1)
                        .required(true)
                        .help("The name of your task\nShould not be named the same as another task")
                )
                .arg(
                    Arg::new("do-date")
                        .index(2)
                        .required(true)
                        .help("When you should do this task")
                )
                .arg(
                    Arg::new("due-date")
                        .index(3)
                        .required(true)
                        .help("When the task is not doable anymore")
                )
                .arg(
                    Arg::new("priority")
                        .index(4)
                        .required(false)
                        .help("How important the task is to get done")
                )
                */
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            print!("Testing input: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            let test = io::stdin().read_line(&mut input);
            println!("You typed: {}", input);
        }
        _ => {

        }
    }
}

fn init() {
    
}
