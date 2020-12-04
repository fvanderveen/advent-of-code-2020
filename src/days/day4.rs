use std::fs::read_to_string;

#[derive(Eq, PartialEq)]
struct Passport {
    byr: Option<String>, // Birth Year
    iyr: Option<String>, // Issue Year
    eyr: Option<String>, // Expiration Year
    hgt: Option<String>, // Height
    hcl: Option<String>, // Hair colour
    ecl: Option<String>, // Eye colour
    pid: Option<String>, // Passport ID
    cid: Option<String>, // Country ID
}

fn read_entry(data: String) -> Passport {
    let pairs = data
        .split("\n")
        .flat_map(|line| line.split(" "))
        .map(|line| line.trim());

    let mut byr = None;
    let mut iyr = None;
    let mut eyr = None;
    let mut hgt = None;
    let mut hcl = None;
    let mut ecl = None;
    let mut pid = None;
    let mut cid = None;

    for pair in pairs {
        match &pair.chars().nth(3) {
            Some(':') => {} // Continue function if we found a colon at the right place.
            Some(_) | None => continue,
        }

        let value = pair[4..].to_owned();

        match &pair[0..3] {
            "byr" => byr = Some(value),
            "iyr" => iyr = Some(value),
            "eyr" => eyr = Some(value),
            "hgt" => hgt = Some(value),
            "hcl" => hcl = Some(value),
            "ecl" => ecl = Some(value),
            "pid" => pid = Some(value),
            "cid" => cid = Some(value),
            _ => {} // Ignore unknown values
        }
    }

    Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid,
    }
}

fn read_input_file() -> Result<Vec<Passport>, String> {
    let data = read_to_string("input/day4.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            // This data is interesting. Entries are separated by a _blank line_. Entries consist of
            // `key:value` pairs separated by whitespace.
            return Ok(data
                .split("\n\n")
                .map(str::to_owned)
                .map(read_entry)
                .collect());
        }
    };
}

fn is_passport_valid1(passport: &Passport) -> bool {
    // All fields, except cid, are required in this implementation.
    match passport {
        Passport {
            byr: Some(_),
            iyr: Some(_),
            eyr: Some(_),
            hgt: Some(_),
            hcl: Some(_),
            ecl: Some(_),
            pid: Some(_),
            ..
        } => true,
        _ => false,
    }
}

pub fn puzzle1() {
    let passports = match read_input_file() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let valid = passports.iter().filter(|&p| is_passport_valid1(p)).count();
    println!("Puzzle 1: We have {} valid passports", valid);
}

fn is_valid_byr(byr: &String) -> bool {
    // BYR => 4 digits (number) and between 1920-2002 incl
    match byr.parse::<i32>() {
        Err(_) => false,
        Ok(v) => v >= 1290 && v <= 2002,
    }
}

fn is_valid_iyr(iyr: &String) -> bool {
    // IYR => 4 digits (number) and between 2010-2020 incl
    match iyr.parse::<i32>() {
        Err(_) => false,
        Ok(v) => v >= 2010 && v <= 2020,
    }
}

fn is_valid_eyr(eyr: &String) -> bool {
    // EYR => 4 digits (number) and between 2020-2030 incl
    match eyr.parse::<i32>() {
        Err(_) => false,
        Ok(v) => v >= 2020 && v <= 2030,
    }
}

fn is_valid_hgt(hgt: &String) -> bool {
    // HGT => number followed by 'in' or 'cm'
    //      'in' => 59-76 incl
    //      'cm' => 150-193 incl
    let len = hgt.len();
    let val = &hgt[0..(len - 2)];
    let unit = &hgt[(len - 2)..];

    match val.parse::<i32>() {
        Err(_) => false,
        Ok(v) => match unit {
            "in" => v >= 59 && v <= 76,
            "cm" => v >= 150 && v <= 193,
            _ => false,
        },
    }
}

fn is_valid_hcl(hcl: &String) -> bool {
    // HCL => HTML color (#1234AB)
    if hcl.len() != 7 || hcl.chars().nth(0).unwrap() != '#' {
        return false;
    }

    hcl[1..].chars().all(|c| match c {
        '0'..='9' | 'a'..='f' => true,
        _ => return false,
    })
}

fn is_valid_ecl(ecl: &String) -> bool {
    // ECL => one of 'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'
    if ecl.len() != 3 {
        return false;
    }

    match &ecl[0..3] {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_valid_pid(pid: &String) -> bool {
    // PID => 9 digit number (may have leading zeroes)
    pid.len() == 9
        && pid.chars().all(|c| match c {
            '0'..='9' => true,
            _ => false,
        })
}

fn is_passport_valid2(passport: &Passport) -> bool {
    // CID is still optional. However:
    // BYR => 4 digits (number) and between 1920-2002 incl
    // IYR => 4 digits (number) and between 2010-2020 incl
    // EYR => 4 digits (number) and between 2020-2030 incl
    // HGT => number followed by 'in' or 'cm'
    //      'in' => 59-76 incl
    //      'cm' => 150-193 incl
    // HCL => HTML color (#1234AB)
    // ECL => one of 'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'
    // PID => 9 digit number (may have leading zeroes)
    match passport {
        Passport {
            byr: Some(byr),
            iyr: Some(iyr),
            eyr: Some(eyr),
            hgt: Some(hgt),
            hcl: Some(hcl),
            ecl: Some(ecl),
            pid: Some(pid),
            ..
        } if is_valid_byr(byr)
            && is_valid_iyr(iyr)
            && is_valid_eyr(eyr)
            && is_valid_hgt(hgt)
            && is_valid_hcl(hcl)
            && is_valid_ecl(ecl)
            && is_valid_pid(pid) =>
        {
            true
        }
        _ => false,
    }
}

pub fn puzzle2() {
    let passports = match read_input_file() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let valid = passports.iter().filter(|&p| is_passport_valid2(p)).count();
    println!("Puzzle 2: We have {} valid passports", valid);
}
