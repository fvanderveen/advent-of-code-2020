use crate::days::day12::Direction::{EAST, NORTH, SOUTH, WEST};
use crate::util::input::read_mapped_input;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    operation: char,
    value: i128,
}

fn parse_line(line: String) -> Result<Instruction, String> {
    let operation = line
        .chars()
        .nth(0)
        .and_then(|c| match c {
            'N' | 'E' | 'S' | 'W' | 'F' | 'L' | 'R' => Some(c),
            _ => None,
        })
        .ok_or(format!(
            "Could not read a valid operation from line '{}'",
            line
        ))?;

    let value = line[1..line.len()]
        .parse::<i128>()
        .map_err(|e| format!("Could not parse value: {}", e))?;

    Ok(Instruction { operation, value })
}

pub fn puzzle1() {
    let lines = match read_mapped_input(12, parse_line) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let mut east: i128 = 0;
    let mut south: i128 = 0;
    let mut direction = EAST;

    for Instruction { operation, value } in lines {
        match operation {
            'N' => south -= value,
            'F' if direction == NORTH => south -= value,
            'E' => east += value,
            'F' if direction == EAST => east += value,
            'S' => south += value,
            'F' if direction == SOUTH => south += value,
            'W' => east -= value,
            'F' if direction == WEST => east -= value,
            'L' => {
                if value % 90 != 0 {
                    eprintln!(
                        "WARN: Directional value is not a multiple of 90 degrees! L{}",
                        value
                    );
                }

                let turns = (value % 360) / 90;
                for _i in 0..turns {
                    direction = match direction {
                        NORTH => WEST,
                        EAST => NORTH,
                        SOUTH => EAST,
                        WEST => SOUTH,
                    };
                }
            }
            'R' => {
                if value % 90 != 0 {
                    eprintln!(
                        "WARN: Directional value is not a multiple of 90 degrees! R{}",
                        value
                    );
                }

                let turns = (value % 360) / 90;
                for _i in 0..turns {
                    direction = match direction {
                        NORTH => EAST,
                        EAST => SOUTH,
                        SOUTH => WEST,
                        WEST => NORTH,
                    };
                }
            }
            _ => { /* ignore, shouldn't be here any way */ }
        }
    }

    // Result of the puzzle is the abs(east) + abs(south)
    println!(
        "Puzzle1: we ended at {} {}, {} {}. Result = {}",
        east.abs(),
        if east < 0 { "west" } else { "east" },
        south.abs(),
        if south < 0 { "north" } else { "south" },
        east.abs() + south.abs()
    )
}

struct Point {
    east: i128,
    south: i128,
}

pub fn puzzle2() {
    let lines = match read_mapped_input(12, parse_line) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let mut waypoint = Point {
        east: 10,
        south: -1,
    };
    let mut location = Point { east: 0, south: 0 };

    for Instruction { operation, value } in lines {
        match operation {
            'N' => waypoint.south -= value,
            'E' => waypoint.east += value,
            'S' => waypoint.south += value,
            'W' => waypoint.east -= value,
            'F' => {
                location.east += waypoint.east * value;
                location.south += waypoint.south * value;
            }
            'L' => {
                if value % 90 != 0 {
                    eprintln!(
                        "WARN: Directional value is not a multiple of 90 degrees! L{}",
                        value
                    );
                }

                let turns = (value % 360) / 90;
                for _i in 0..turns {
                    // A turn left means we'll need to rotate the waypoint
                    // east -> north (= east -> -south)
                    // north -> west (= -south -> -east)
                    // west -> south (= -east -> south)
                    // south -> east (= south -> east)
                    waypoint = Point {
                        east: waypoint.south,
                        south: -waypoint.east,
                    }
                }
            }
            'R' => {
                if value % 90 != 0 {
                    eprintln!(
                        "WARN: Directional value is not a multiple of 90 degrees! R{}",
                        value
                    );
                }

                let turns = (value % 360) / 90;
                for _i in 0..turns {
                    // A turn right means we'll need to rotate the waypoint
                    // east -> south (= east -> south)
                    // north -> east (= -south -> east)
                    // west -> north (= -east -> -south)
                    // south -> west (= south -> -east)
                    waypoint = Point {
                        east: -waypoint.south,
                        south: waypoint.east,
                    }
                }
            }
            _ => { /* ignore, shouldn't be here any way */ }
        }
    }

    // Result of the puzzle is the abs(east) + abs(south)
    println!(
        "Puzzle2: we ended at {} {}, {} {}. Result = {}",
        location.east.abs(),
        if location.east < 0 { "west" } else { "east" },
        location.south.abs(),
        if location.east < 0 { "north" } else { "south" },
        location.east.abs() + location.south.abs()
    )
}
