use std::fs::read_to_string;

pub fn read_raw_input(day: i32) -> Result<String, String> {
    read_to_string(format!("input/day{}.txt", day)).map_err(|e| format!("{}", e))
}

pub fn read_numeric_input(day: i32) -> Result<Vec<i128>, String> {
    read_mapped_input(day, to_number)
}

pub fn read_mapped_input<T, F: FnMut(String) -> Result<T, String>>(
    day: i32,
    mutator: F,
) -> Result<Vec<T>, String> {
    read_raw_input(day).and_then(|d| d.split("\n").map(str::to_owned).map(mutator).collect())
}

pub fn to_number(line: String) -> Result<i128, String> {
    line.parse().map_err(|e| format!("{}", e))
}
