mod task;
mod list;

use std::fs;

use clap::Parser;

use task::Date;
use task::DateSpec;

fn main() {
    // Get file
    let file_name = "tasks.txt";

    let contents = fs::read_to_string(file_name).unwrap_or_else(|err| {
        let file = fs::File::create(file_name);
        "".to_string()
    });

    let current_time = Date::new(11, 12, 2005);
    current_time.print();

    let do_date = DateSpec::Single(&current_time);
    do_date.print();

    let end_date = Date::new(4, 12, 2014);
    let due_date = DateSpec::Range(&current_time, &end_date);
    due_date.print();

    do_date.print();
}

fn init() {
    
}
