pub struct Task {

}

enum DateSpec {
    Single(Date),
    Range(Date, Date),
}

impl DateSpec {
    pub fn new_single(date: Date) -> Self {
        DateSpec::Single(date)
    }

    pub fn new_range(start: Date, end: Date) -> Self {
        DateSpec::Range(start, end)
    }
    
    pub fn print(&self) {
        
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
        println!("{}-{}-{}", self.month, self.day, self.year);
    }
}