use do_due_cli::*;

use clap::{Command, command};

fn main() {

    let config = Config::build("tasks.txt");
    
    let matches = command!()
        .subcommand(
    Command::new("add")
                .about("Add a new task")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            let name = get_input("Enter the name of the task: ")
                .unwrap_or_else(|_| "Error processing input".to_string());

            let do_date = get_input("Enter the do date of the task: ")
                .expect("Problem getting do date");

            let due_date = get_input("Enter the due date of the task: ")
                .expect("Problem getting due date");

            let priority = get_input("Enter the priority of the task: ")
                .expect("Problem getting priority");

            let description = get_input("Enter the description of the task: ")
                .expect("Problem getting description");

            println!("You typed: {}", name);
        }
        _ => {

        }
    }
}