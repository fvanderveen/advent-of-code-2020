extern crate lazy_static;

mod days {
    pub mod day1;
    pub mod day2;
}

fn main() {
    days::day1::puzzle1();
    days::day1::puzzle2();

    days::day2::puzzle1();
    days::day2::puzzle2();
}
