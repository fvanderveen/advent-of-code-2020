extern crate lazy_static;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}

fn main() {
    let day = 3;

    match day {
        1 => {
            days::day1::puzzle1();
            days::day1::puzzle2();
        }
        2 => {
            days::day2::puzzle1();
            days::day2::puzzle2();
        }
        3 => {
            days::day3::puzzle1();
            days::day3::puzzle2();
        }
        _ => {}
    }
}
