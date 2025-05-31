pub struct Task {
    pub name: String,
    pub do_date: DateSpec,
    pub due_date: DateSpec,
    pub desc: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(name: String, do_date: DateSpec, due_date: DateSpec, desc: String, priority: Priority) -> Self {
        Task {
            due_date,
            do_date,
            name,
            desc,
            priority,
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        format!("Task: {} | {} | {}", self.name, self.do_date.to_string(), self.due_date.to_string())
    }
}

pub enum Priority {
    Low,
    Medium,
    High,
}

pub enum DateSpec {
    Single(Date),
    Range(Date, Date),
}

impl DateSpec {
    pub fn print(&self) {
        match self {
            Self::Single(date) => println!("single type: {}", date.to_string()),
            Self::Range(start, end) => println!("range type: {}-{}", start.to_string(), end.to_string()),
        }   
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Single(date) => format!("{}", date.to_string()),
            Self::Range(start, end) => format!("{} - {}", start.to_string(), end.to_string()),
        }  
    }

    pub fn validate(input: String) -> Result<DateSpec, &'static str> {
        if input.len() != 10 || input.len() != 21 {
            return Err("Input not valid.");
        }
    }
}

pub struct Date {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl Date {
    pub fn new(month: u8, day: u8, year: u16) -> Self {
        Date {
            month,
            day,
            year,
        }
    }

    pub fn print(&self) {
        println!("{}/{}/{}", self.month, self.day, self.year);
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}/{}", self.month, self.day, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        
    }
}