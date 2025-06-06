use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process;

pub mod task;
use task::Task;
use task::DateSpec;
use task::Priority;

pub struct Config {
    pub file: File,
    pub list: Vec<Task>,
}

impl Config {
    pub fn build(path: &str) -> Config {
        let file: File = File::open(path).unwrap_or_else(|_| {
            File::create(path).expect("Couldn't create a file")
        });

        let mut list: Vec<Task> = Vec::new();

        let reader: BufReader<&File> = BufReader::new(&file);

        for line_result in reader.lines() {
            let line = line_result.unwrap_or_else(|err|{
                eprintln!("I/O Error, might need to reset tasks: {}", err.to_string());
                process::exit(1);
            });

            let components: Vec<&str> = line.split("|").collect();

            if components.len() != 5 {
                continue;
            }
            
            let name = components[0];
            let do_date = DateSpec::validate(components[1]).unwrap_or_else(|err| {
                eprintln!("Can't initialize program 1, reset program: {}", err);
                process::exit(1);
            });

            let due_date = DateSpec::validate(components[2]).unwrap_or_else(|_| {
                eprintln!("Can't initialize program 2, reset program.");
                process::exit(1);
            });

            let priority = Priority::validate(components[3]);
            let desc = components[4];

            let task = Task::new(name.to_string(), do_date, due_date, desc.to_string(), priority);

            list.push(task);
        }

        Config {
            file,
            list,
        }
    }

    pub fn insert(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn delete(&mut self, task_name: String) {
        let mut count = 0;
        for task in &self.list {
            if task.name == task_name {
                self.list.remove(count);
                break;
            }
            count += 1;
        }
    }

    pub fn print(&self) {
        for task in &self.list {
            task.print();
        }
    }

    pub fn end(&mut self) -> std::io::Result<()> {
        self.file = File::create("tasks.txt")?;

        for task in &self.list {
            writeln!(self.file, "{}", task.to_string())?;
        }

        Ok(())
    }
}

pub fn get_input(msg: &str) -> Result<String, String> {
    print!("{}", msg);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    let test: Result<usize, io::Error> = io::stdin().read_line(&mut input);

    match test {
        Err(e) => return Err(format!("Problem getting input: {}", e)),
        _ => ()
    }

    if input.len() < 1 {
        return Err("Input can't be empty".to_string());
    }

    Ok(input)
}

pub fn get_input_empty(msg: &str) -> Result<String, String> {
    print!("{}", msg);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    let test: Result<usize, io::Error> = io::stdin().read_line(&mut input);

    match test {
        Err(e) => return Err(format!("Problem getting input: {}", e)),
        _ => ()
    }

    Ok(input)
}
