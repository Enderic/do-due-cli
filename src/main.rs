use do_due_cli::*;

use clap::{Command, command};

use task::Task;


fn main() {
    let v: Vec<Task> = Vec::new();
    let matches = command!()
        .subcommand(
    Command::new("add")
                .about("Add a new task")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            let input = get_input("Enter the name of the task: ")
                .unwrap_or_else(|_| "Error getting input".to_string());

            println!("You typed: {}", input);
        }
        _ => {

        }
    }
}