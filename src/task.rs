pub struct Task<'a, 'b> {
    pub due_date: DateSpec<'a, 'b>,
    pub do_date: DateSpec<'a, 'b>,
    pub name: &'a str,
    pub desc: &'a str,
    pub priority: Priority,
}

pub enum Priority {
    Low,
    Medium,
    High,
}

pub enum DateSpec<'a, 'b> {
    Single(&'a Date),
    Range(&'a Date, &'b Date),
}

impl<'a, 'b> DateSpec<'a, 'b> {
    pub fn print(&self) {
        match self {
            Self::Single(date) => println!("single type: {}", date.to_string()),
            Self::Range(start, end) => println!("range type: {}-{}", start.to_string(), end.to_string()),
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