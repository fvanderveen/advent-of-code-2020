use crate::days::day14::Mode::{CLEAR, FLUX, SET};
use crate::util::input::read_raw_input;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Mode {
    SET,
    CLEAR,
    FLUX,
}

#[derive(Eq, PartialEq, Debug)]
struct Mask {
    value: u128,
    mode: Mode,
}

fn read_memory_mask(mask: &str) -> Vec<Mask> {
    let mut result = vec![];
    let mut pos = 0;

    for c in mask.chars().rev() {
        match c {
            '1' => result.push(Mask {
                value: 1 << pos,
                mode: SET,
            }),
            '0' => result.push(Mask {
                value: 1 << pos,
                mode: CLEAR,
            }),
            'X' => result.push(Mask {
                value: 1 << pos,
                mode: FLUX,
            }),
            _ => {} // Nothing to
        }
        pos += 1
    }

    result
}

fn apply_mask(value: u128, mask: &Vec<Mask>) -> u128 {
    let mut result = value;

    for m in mask {
        match m.mode {
            SET => result |= m.value,
            CLEAR => result &= !m.value, // TIL: !<num> in rust is the same as ~<num> in anything else :sweat:
            FLUX => {}                   // Not needed for puzzle 1
        }
    }

    result
}

#[test]
fn test_mask() {
    let mask = read_memory_mask(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    println!("{:?}", mask);
    assert_eq!(apply_mask(11, &mask), 73);
    assert_eq!(apply_mask(101, &mask), 101);
    assert_eq!(apply_mask(0, &mask), 64);
}

pub fn puzzle1() {
    let lines =
        match read_raw_input(14).map(|d| d.split("\n").map(str::to_owned).collect::<Vec<_>>()) {
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
            Ok(v) => v,
        };

    let mut memory: HashMap<String, u128> = HashMap::new();
    let mut mask: Vec<Mask> = vec![];

    for line in lines {
        // Line can be two things:
        // mask = [X01]+ => representing the new bitmask
        // mem\[\d+\] = \d+ => set value at memory
        if line[0..7].eq("mask = ") {
            mask = read_memory_mask(&line[7..]);
        }

        if line[0..4].eq("mem[") {
            let address = line
                .chars()
                .skip(4)
                .take_while(|c| c.ne(&']'))
                .collect::<String>();
            let value = match line
                .chars()
                .skip(5 + address.len())
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u128>()
            {
                Err(e) => {
                    eprintln!("Could not parse input '{}': {}", line, e);
                    return;
                }
                Ok(v) => v,
            };

            memory.insert(address, apply_mask(value, &mask));
        }
    }

    let result: u128 = memory.values().sum();
    println!("Puzzle 1: sum of all stored memory values = {}", result);
}

fn write_value(memory: &mut HashMap<u128, u128>, address: u128, value: u128, mask: &Vec<Mask>) {
    let mut base_address = address;
    // Apply all SET bits to base_address:
    mask.iter()
        .filter(|m| match m.mode {
            SET => true,
            _ => false,
        })
        .for_each(|m| base_address |= m.value);

    // Now, for all FLUX bits, we need to write all combinations of 0 and 1 possible.
    write_flux_values(memory, base_address, value, &mask);
}

fn write_flux_values(memory: &mut HashMap<u128, u128>, address: u128, value: u128, mask: &[Mask]) {
    // If we have no more flux bits, write the value
    if mask.len() == 0 {
        memory.insert(address, value);
        return;
    }

    // Otherwise, recurse for the current flux bit's on and off value:
    let bit = &mask[0];
    let rest = &mask[1..];

    match bit.mode {
        FLUX => {
            write_flux_values(memory, address | bit.value, value, rest);
            write_flux_values(memory, address & !bit.value, value, rest);
        }
        _ => {
            // Since filtering and calling this function with only FLUX bits is a pain, this'll do.
            write_flux_values(memory, address, value, rest);
        }
    }
}

#[test]
fn test_write_flux_values() {
    let mut memory = HashMap::new();
    let mask = read_memory_mask("000000000000000000000000000000X1001X");
    write_value(&mut memory, 42, 100, &mask);
    println!(
        "Keys in map: {:?}",
        memory.keys().map(|k| format!("{}", k)).collect::<Vec<_>>()
    );
    assert_eq!(memory.len(), 4);
    assert_eq!(memory.get(&26), Some(&100));
    assert_eq!(memory.get(&27), Some(&100));
    assert_eq!(memory.get(&58), Some(&100));
    assert_eq!(memory.get(&59), Some(&100));
}

pub fn puzzle2() {
    let lines =
        match read_raw_input(14).map(|d| d.split("\n").map(str::to_owned).collect::<Vec<_>>()) {
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
            Ok(v) => v,
        };

    // So,this time, the mask works quite differently.
    // It'll mutate the memory address by the following rules:
    // 0 => does nothing
    // 1 => force to 1
    // X => write for both 0 and 1 values.
    let mut memory: HashMap<u128, u128> = HashMap::new();
    let mut mask: Vec<Mask> = vec![];

    for line in lines {
        // Line can be two things:
        // mask = [X01]+ => representing the new bitmask
        // mem\[\d+\] = \d+ => set value at memory
        if line[0..7].eq("mask = ") {
            mask = read_memory_mask(&line[7..]);
        }

        if line[0..4].eq("mem[") {
            let address = match line
                .chars()
                .skip(4)
                .take_while(|c| c.ne(&']'))
                .collect::<String>()
                .parse::<u128>()
            {
                Err(e) => {
                    eprintln!("Could not parse address '{}': {}", line, e);
                    return;
                }
                Ok(v) => v,
            };

            let value = match line
                .chars()
                .skip_while(|c| c.ne(&'=')) // Skip until the assignment op
                .skip_while(|c| !c.is_numeric()) // Skip any non-numeric
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u128>()
            {
                Err(e) => {
                    eprintln!("Could not parse input '{}': {}", line, e);
                    return;
                }
                Ok(v) => v,
            };

            write_value(&mut memory, address, value, &mask);
        }
    }

    let result: u128 = memory.values().sum();
    println!("Puzzle 2: sum of all stored memory values = {}", result);
}
