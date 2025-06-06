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
        format!("{}|{}|{}|{}|{}", self.name.trim(), self.do_date.to_string(), self.due_date.to_string(), self.priority.as_str(), self.desc.trim())
    }
}

pub enum Priority {
    High,
    Medium,
    Low,
    None,
}

impl Priority {
    pub fn validate(input: &str) -> Priority {
        match input {
            "High" => Priority::High,
            "Medium" => Priority::Medium,
            "Low" => Priority::Low,
            _ => Priority::None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
            Priority::None => "None",
        }
    }
}

pub enum DateSpec {
    Single(Date),
    Range(Date, Date),
}

impl DateSpec {
    pub fn validate(input: &str) -> Result<DateSpec, String> {
        let input = input.trim();

        // Range format: MM/DD/YYYY-MM/DD/YYYY or M/D/YYYY-M/D/YYYY
        if input.contains('-') {
            let dates: Vec<&str> = input.split('-').collect();
            if dates.len() != 2 {
                return Err("Range must contain exactly one '-' between two dates".to_string());
            }

            let start = DateSpec::parse_date(dates[0].trim())?;
            let end = DateSpec::parse_date(dates[1].trim())?;
            return Ok(DateSpec::Range(start, end));
        }

        // Single format: MM/DD/YYYY or M/D/YYYY
        let single = DateSpec::parse_date(input)?;
        Ok(DateSpec::Single(single))
    }

    fn parse_date(date_str: &str) -> Result<Date, String> {
        let parts: Vec<&str> = date_str.trim().split('/').collect();
        if parts.len() != 3 {
            return Err("Date must be in M/D/YYYY or MM/DD/YYYY format".to_string());
        }

        let month: u8 = parts[0].parse().map_err(|_| "Invalid month".to_string())?;
        let day: u8 = parts[1].parse().map_err(|_| "Invalid day".to_string())?;
        let year: u16 = parts[2].parse().map_err(|_| "Invalid year".to_string())?;

        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return Err("Date values out of range".to_string());
        }

        Ok(Date::new(month, day, year))
    }

    pub fn print(&self) {
        match self {
            Self::Single(date) => println!("single type: {}", date.to_string()),
            Self::Range(start, end) => println!("range type: {}-{}", start.to_string(), end.to_string()),
        }   
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Single(date) => format!("{}", date.to_string()),
            Self::Range(start, end) => format!("{}-{}", start.to_string(), end.to_string()),
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

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        
    }
}
*/