use std::process;

use do_due_cli::{task::{DateSpec, Priority, Task}, *};

use clap::{command, Arg, Command};

fn main() {

    let mut config: Config = Config::build("tasks.txt");
    
    let matches = command!()
        .subcommand(
    Command::new("add")
                .about("Add a new task")
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes an existing task")
                .arg(
                    Arg::new("name")
                        .index(1)
                        .required(true)
                        .help("The name of the task to delete")
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists all tasks & their details")
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
        },
        Some(("delete", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            config.delete(name.clone());
        }
        _ => ()
    }

    let _ = config.end();
}