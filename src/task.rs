pub struct Task {

}

enum DateSpec {
    Single(Date),
    Range(Date, Date),
}

impl DateSpec {

}

pub struct Date {
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl Date {
    pub fn new(month: u8, day: u8, year: u16) -> Date {
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