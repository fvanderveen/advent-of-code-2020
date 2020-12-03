use std::fs::{read_to_string};

fn read_input_file() -> Result<Vec<i32>, String> {
    let data = read_to_string("input/day1.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            let results: Result<Vec<_>, _> = data.split("\n").map(|l| l.parse::<i32>()).collect();
            match results {
                Err(e) => Err(format!("Could not parse all lines? {}", e)),
                Ok(numbers) => Ok(numbers)
            }
        }
    };
}

struct Pair (i32, i32);

fn find_pair(input: &Vec<i32>, search: i32) -> Result<Pair, String> {
    if input.len() < 2 {
        return Err(format!("Could not find pair of numbers with sum {}", search));
    }

    let first = input[0];
    let rest = input[1..].to_vec();

    for &second in &rest {
        if first + second == search {
            return Ok(Pair(first, second));
        }
    }

    return find_pair(&rest, search);
}

pub fn puzzle1() {
    let numbers = match read_input_file() {
        Ok(numbers) => numbers,
        Err(e) => {
            println!("Could not read/parse file: {}", e);
            return;
        }
    };

    // Now that we have our list of numbers, we need to find two numbers (x, y) => x + y = 2020
    // The result we are after is: x * y
    match find_pair(&numbers, 2020) {
        Err(e) => println!("{}", e),
        Ok(Pair(first, second)) => {
            println!("Found 2020 in {} + {}", first, second);
            println!("Solution to puzzle1: {}", first * second);
        }
    }
}

struct Triple(i32, i32, i32);

fn find_triplet(input: &Vec<i32>, search: i32) -> Result<Triple, String> {
    if input.len() < 3 {
        return Err(format!("Could not find triplet of numbers with sum {}", search));
    }

    let first = input[0];
    let rest = input[1..].to_vec();

    return match find_pair(&rest, search - first) {
        Ok(Pair(second, third)) => Ok(Triple(first, second, third)),
        Err(_) => find_triplet(&rest, search)
    }
}

pub fn puzzle2() {
    let numbers = match read_input_file() {
        Ok(numbers) => numbers,
        Err(e) => {
            println!("Could not read/parse file: {}", e);
            return;
        }
    };

    match find_triplet(&numbers, 2020) {
        Err(e) => println!("{}", e),
        Ok(Triple(first, second, third)) => {
            println!("Found 2020 in {} + {} + {}", first, second, third);
            println!("Solution to puzzle1: {}", first * second * third);
        }
    }
}