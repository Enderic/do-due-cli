pub struct Task {
    pub due_date: DateSpec,
    pub do_date: DateSpec,
    pub name: String,
    pub desc: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(due_date: DateSpec, do_date: DateSpec, name: String, desc: String, priority: Priority) -> Self {
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