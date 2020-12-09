use std::fs::read_to_string;

fn read_input_file() -> Result<String, String> {
    let data = read_to_string("input/day9.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            return Ok(data);
        }
    };
}

fn parse_data(data: String) -> Result<Vec<usize>, String> {
    data.split("\n")
        .map(|l| l.parse::<usize>().map_err(|e| format!("{}", e)))
        .collect()
}

fn has_sum(list: &[usize], sum: usize) -> bool {
    if list.len() == 0 {
        return false;
    }

    let first = list[0];
    let rest = &list[1..];

    rest.iter().any(|i| i + first == sum) || has_sum(rest, sum)
}

fn find_first_invalid_number(list: &Vec<usize>, preamble: usize) -> Option<usize> {
    // The first 25 numbers are preamble, any next number is valid iff there exist two numbers in the
    // last 25 numbers that sum to it.

    for i in preamble + 1..list.len() {
        if !has_sum(&list[i - preamble - 1..i], list[i]) {
            return Some(list[i]);
        }
    }

    None
}

#[test]
fn test_find_first_invalid_number() {
    let list = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(find_first_invalid_number(&list, 5), Some(127));

    let fail = vec![
        105, 82, 58, 68, 61, 84, 101, 116, 70, 125, 78, 81, 199, 83, 100, 87, 88, 170, 169, 108,
        140, 107, 193, 139, 168, 229,
    ];
    assert_eq!(fail.len(), 26);
    assert_eq!(find_first_invalid_number(&fail, 25), None);
}

pub fn puzzle1() {
    let list = match read_input_file().and_then(parse_data) {
        Ok(v) => v,
        Err(e) => {
            println!("Could not read/parse file: {}", e);
            return;
        }
    };

    let result = find_first_invalid_number(&list, 25);
    match result {
        Some(v) => println!("Puzzle 1: First invalid number = {}", v),
        None => println!("Puzzle 1: Did not find invalid numbers?"),
    }
}

fn find_contiguous_set(list: &[usize], sum: usize) -> Option<&[usize]> {
    let mut lower = 0;
    let mut upper = 1;

    loop {
        let current: usize = list[lower..upper].iter().sum();
        if current == sum {
            return Some(&list[lower..upper]);
        }

        if current < sum {
            // Increase upper search bound
            upper += 1;
        } else {
            // If we're a bigger number now, we reset to the next lower bound
            lower += 1;
            upper = lower + 1;
        }

        if upper >= list.len() {
            lower += 1;
            upper = lower + 1;
        }

        if lower + 1 >= list.len() {
            return None;
        }
    }
}

pub fn puzzle2() {
    let list = match read_input_file().and_then(parse_data) {
        Ok(v) => v,
        Err(e) => {
            println!("Could not read/parse file: {}", e);
            return;
        }
    };

    // We need to find a contiguous set of numbers that sum to the answer of puzzle 1: 26796446
    let result = find_contiguous_set(&list, 26796446);

    match result {
        None => println!("Could not seem to find a contiguous set..."),
        Some(v) => {
            // Result will be the sum of the first and last number:
            println!(
                "Found a result: {} => {}",
                v.iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                v.iter().sum::<usize>()
            );

            let min = v.iter().min();
            let max = v.iter().max();
            match (min, max) {
                (Some(v1), Some(v2)) => println!("Puzzle 2 result: {}", v1 + v2),
                _ => println!("Set has no first/last entry?!"),
            }
        }
    }
}
