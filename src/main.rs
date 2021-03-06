extern crate lazy_static;

use std::io::{stdin, stdout, Write};

mod days;
mod util;

fn main() {
    let latest_day = 25;

    let mut input = String::new();
    print!(
        "Please enter the day to run and press enter [{}]: ",
        latest_day
    );

    if let Err(e) = stdout().flush() {
        eprintln!("{}", e);
        return;
    }

    match stdin().read_line(&mut input) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(_) => {}
    }

    let day = if input.trim().len() == 0 {
        Ok(latest_day)
    } else {
        input.trim().parse::<i32>()
    };

    match day {
        Ok(1) => {
            days::day01::puzzle1();
            days::day01::puzzle2();
        }
        Ok(2) => {
            days::day02::puzzle1();
            days::day02::puzzle2();
        }
        Ok(3) => {
            days::day03::puzzle1();
            days::day03::puzzle2();
        }
        Ok(4) => {
            days::day04::puzzle1();
            days::day04::puzzle2();
        }
        Ok(5) => {
            days::day05::puzzle1();
            days::day05::puzzle2();
        }
        Ok(6) => {
            days::day06::puzzle1();
            days::day06::puzzle2();
        }
        Ok(7) => {
            days::day07::puzzle1();
            days::day07::puzzle2();
        }
        Ok(8) => {
            days::day08::puzzle1();
            days::day08::puzzle2();
        }
        Ok(9) => {
            days::day09::puzzle1();
            days::day09::puzzle2();
        }
        Ok(10) => {
            days::day10::puzzle1();
            days::day10::puzzle2();
        }
        Ok(11) => {
            days::day11::puzzle1();
            days::day11::puzzle2();
        }
        Ok(12) => {
            days::day12::puzzle1();
            days::day12::puzzle2();
        }
        Ok(13) => {
            days::day13::puzzle1();
            days::day13::puzzle2();
        }
        Ok(14) => {
            days::day14::puzzle1();
            days::day14::puzzle2();
        }
        Ok(15) => {
            days::day15::puzzle1();
            days::day15::puzzle2();
        }
        Ok(16) => {
            days::day16::puzzle1();
            days::day16::puzzle2();
        }
        Ok(17) => {
            days::day17::puzzle1();
            days::day17::puzzle2();
        }
        Ok(18) => {
            days::day18::puzzle1();
            days::day18::puzzle2();
        }
        Ok(19) => {
            days::day19::puzzle1();
            days::day19::puzzle2();
        }
        Ok(20) => {
            days::day20::puzzle1();
            days::day20::puzzle2();
        }
        Ok(21) => {
            days::day21::puzzle1();
            days::day21::puzzle2();
        }
        Ok(22) => {
            days::day22::puzzle1();
            days::day22::puzzle2();
        }
        Ok(23) => {
            days::day23::puzzle1();
            days::day23::puzzle2();
        }
        Ok(24) => {
            days::day24::puzzle1();
            days::day24::puzzle2();
        }
        Ok(25) => {
            days::day25::puzzle1();
            days::day25::puzzle2();
        }
        Ok(v) => eprintln!("I don't know about day {}", v),
        Err(e) => eprintln!("{}", e),
    }
}
