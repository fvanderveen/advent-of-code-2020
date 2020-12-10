extern crate lazy_static;

mod days {
    pub mod day1;
    pub mod day10;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

fn main() {
    let day = 10;

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
        7 => {
            days::day7::puzzle1();
            days::day7::puzzle2();
        }
        8 => {
            days::day8::puzzle1();
            days::day8::puzzle2();
        }
        9 => {
            days::day9::puzzle1();
            days::day9::puzzle2();
        }
        10 => {
            days::day10::puzzle1();
            days::day10::puzzle2();
        }
        _ => {}
    }
}
