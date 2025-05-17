mod task;

use task::Date;
fn main() {
    let current_time = Date::new(11, 12, 2005);
    current_time.print();
}
