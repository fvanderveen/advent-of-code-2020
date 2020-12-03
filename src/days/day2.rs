use regex::Regex;
use std::fmt;
use std::fs::read_to_string;

struct Policy {
    min: i32,
    max: i32,
    letter: char,
}

struct DbEntry(Policy, String);

impl fmt::Display for DbEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} {}: {}",
            self.0.min, self.0.max, self.0.letter, self.1
        )
    }
}

struct ParseError(String);

fn parse_db_entry(line: &str) -> Result<DbEntry, ParseError> {
    // Line format:
    // \d+-\d+\s+[a-z]:\s+[a-z]+
    let format =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s+(?P<char>[a-z]):\s+(?P<password>[a-z]+)$")
            .map_err(|e| ParseError(e.to_string()))?;

    format
        .captures(line)
        .map(|mat| {
            let min = mat
                .name("min")
                .map(|v| v.as_str().parse::<i32>())
                .ok_or(ParseError("No match for min".to_string()))?
                .map_err(|ipe| ParseError(ipe.to_string()))?;
            let max = mat
                .name("max")
                .map(|v| v.as_str().parse::<i32>())
                .ok_or(ParseError("No match for max".to_string()))?
                .map_err(|ipe| ParseError(ipe.to_string()))?;
            let letter: char = mat
                .name("char")
                .map(|v| v.as_str().chars().nth(0))
                .ok_or(ParseError("No match for char".to_string()))?
                .ok_or(ParseError("No first char".to_string()))?;
            let password = mat
                .name("password")
                .map(|v| v.as_str())
                .ok_or(ParseError("No match for password".to_string()))?;

            Ok(DbEntry(Policy { min, max, letter }, password.to_string()))
        })
        .ok_or(ParseError(format!("Regex match failure, '{}'", line)))?
}

fn read_input_file() -> Result<Vec<DbEntry>, String> {
    let data = read_to_string("input/day2.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            let results: Result<Vec<_>, _> = data.split("\n").map(parse_db_entry).collect();
            match results {
                Err(ParseError(details)) => Err(format!("Could not parse all lines: {}", details)),
                Ok(entries) => Ok(entries),
            }
        }
    };
}

fn password_valid(DbEntry(policy, password): &DbEntry) -> bool {
    let target = policy.letter;

    let mut count = 0;
    for char in password.chars() {
        if char == target {
            count += 1;
        }
    }

    return count >= policy.min && count <= policy.max;
}

pub fn puzzle1() {
    let entries = match read_input_file() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let mut valid = 0;
    for entry in entries {
        if password_valid(&entry) {
            valid += 1;
        }
    }

    println!("P1: Found {} valid password(s)", valid);
}

fn password_valid2(DbEntry(policy, password): &DbEntry) -> bool {
    // New policy.. `min` and `max` are (1-based!) indexes. Exactly one of them must be `letter`
    let char1 = password.chars().nth((policy.min - 1) as usize).unwrap();
    let char2 = password.chars().nth((policy.max - 1) as usize).unwrap();

    return char1 != char2 && (char1 == policy.letter || char2 == policy.letter);
}

pub fn puzzle2() {
    let entries = match read_input_file() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let mut valid = 0;
    for entry in entries {
        if password_valid2(&entry) {
            valid += 1;
        }
    }

    println!("P2: Found {} valid password(s)", valid);
}
