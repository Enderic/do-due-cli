mod task;

use task::Date;
use task::DateSpec;

fn main() {
    let current_time = Date::new(11, 12, 2005);
    current_time.print();

    let do_date = DateSpec::Single(&current_time);
    do_date.print();

    let end_date = Date::new(4, 12, 2014);
    let due_date = DateSpec::Range(&current_time, &end_date);
    due_date.print();

    do_date.print();
}
