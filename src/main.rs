mod task;

use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use clap::Parser;

use task::Task;
use task::Date;
use task::DateSpec;

fn main() {
    // Get file
    let file_name = "tasks.txt";

    let file = fs::File::open(file_name).unwrap_or_else(|err| {
        fs::File::create(file_name).unwrap()
    });

    let task_list: Vec<Task> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            println!("Line: {}", l);
        }
    }

    /*
    let current_time = Date::new(11, 12, 2005);
    current_time.print();

    let do_date = DateSpec::Single(&current_time);
    do_date.print();

    let end_date = Date::new(4, 12, 2014);
    let due_date = DateSpec::Range(&current_time, &end_date);
    due_date.print();

    do_date.print();
    */

    /* 
    Add task:
        - Ask for name
        - Ask for due date
        - Ask for do date
        - Ask for description
        - Ask for priority
        - Create task & put in list
    */
}

fn init() {
    
}
