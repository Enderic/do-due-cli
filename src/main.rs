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
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _sub_m)) => {
            print!("Testing input: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            let _test = io::stdin().read_line(&mut input);
            println!("You typed: {}", input);
        }
        _ => {

        }
    }
}

fn init() {
    
}
