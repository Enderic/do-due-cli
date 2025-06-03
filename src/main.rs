use std::process;

use do_due_cli::{task::{DateSpec, Priority, Task}, *};

use clap::{Command, command};

fn main() {

    let mut config: Config = Config::build("tasks.txt");
    
    let matches = command!()
        .subcommand(
    Command::new("add")
                .about("Add a new task")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            let name: String = get_input("Enter the name of the task: ").unwrap_or_else(|err: String| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let do_date_input: String = get_input("Enter the do date of the task (Format should be MM/DD/YYYY or MM/DD/YYYY-MM/DD/YYYY: ").unwrap_or_else(|err: String| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let do_date = DateSpec::validate(&do_date_input).unwrap_or_else(|err| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let due_date_input: String = get_input("Enter the due date of the task (Format should be MM/DD/YYYY or MM/DD/YYYY-MM/DD/YYYY: ").unwrap_or_else(|err: String| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let due_date = DateSpec::validate(&due_date_input).unwrap_or_else(|_| {
                eprintln!("Error processing input.");
                process::exit(1);
            });

            let priority_input: String = get_input("Enter the priority of the task (High, Medium, Low, or None, any bad input will default to None: ").unwrap_or_else(|err: String| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let priority = Priority::validate(&priority_input);

            let desc: String = get_input_empty("Enter the description of the task: ").unwrap_or_else(|err: String| {
                eprintln!("Error processing input: {err}");
                process::exit(1);
            });

            let task = Task::new(name, do_date, due_date, desc, priority);

            config.insert(task);
        }
        _ => {

        }
    }
}