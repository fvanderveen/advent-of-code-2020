extern crate lazy_static;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
}

fn main() {
    let day = 6;

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
        4 => {
            days::day4::puzzle1();
            days::day4::puzzle2();
        }
        5 => {
            days::day5::puzzle1();
            days::day5::puzzle2();
        }
        6 => {
            days::day6::puzzle1();
            days::day6::puzzle2();
        }
        _ => {}
    }
}
