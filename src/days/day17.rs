// Okay... how to somewhat represent an ever-growing 3d space in rust :thinking:

use crate::days::day17::State::{ACTIVE, INACTIVE};
use crate::util::input::read_raw_input;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum State {
    INACTIVE,
    ACTIVE,
}

fn create_initial_state() -> Result<HashMap<Location, State>, String> {
    let data = read_raw_input(17)?;
    Ok(parse_map(data, 0))
}

fn parse_map(data: String, z: i32) -> HashMap<Location, State> {
    let mut state = HashMap::new();
    let lines = data.split("\n").map(str::to_owned).collect::<Vec<_>>();
    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            match line.chars().nth(x) {
                Some('.') => {
                    state.insert(
                        Location {
                            x: x as i32,
                            y: y as i32,
                            z,
                        },
                        INACTIVE,
                    );
                }
                Some('#') => {
                    state.insert(
                        Location {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                        },
                        ACTIVE,
                    );
                }
                _ => { /* ignore */ }
            }
        }
    }
    state
}

fn get_surrounding_locations(cell: &Location) -> Vec<Location> {
    let mut result = vec![];
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if z == 0 && y == 0 && x == 0 {
                    continue;
                }
                result.push(Location {
                    x: cell.x + x,
                    y: cell.y + y,
                    z: cell.z + z,
                })
            }
        }
    }
    result
}

fn get_new_state(cells: &HashMap<Location, State>, cell: &Location) -> State {
    // We'll need to consider all (26) direct neighbours (also diagonally)
    // The following applies:
    // If cell is active => remains active iff 2 or 3 neighbours are active
    // If cell is inactive => becomes active iff 3 neighbours are active

    let initial_state = match cells.get(cell) {
        Some(state) => *state,
        None => INACTIVE,
    };

    let active_cells_around: i32 = get_surrounding_locations(cell)
        .iter()
        .map(|l| match cells.get(l) {
            Some(ACTIVE) => 1,
            _ => 0,
        })
        .sum();

    match initial_state {
        ACTIVE if active_cells_around == 2 || active_cells_around == 3 => ACTIVE,
        INACTIVE if active_cells_around == 3 => ACTIVE,
        _ => INACTIVE,
    }
}

#[test]
fn test_get_new_state() {
    let mut cells = HashMap::new();
    cells.insert(Location { x: 0, y: 0, z: 0 }, INACTIVE);
    cells.insert(Location { x: 0, y: 1, z: 0 }, ACTIVE);
    cells.insert(Location { x: 0, y: 2, z: 0 }, INACTIVE);
    cells.insert(Location { x: 1, y: 0, z: 0 }, INACTIVE);
    cells.insert(Location { x: 1, y: 1, z: 0 }, INACTIVE);
    cells.insert(Location { x: 1, y: 2, z: 0 }, ACTIVE);
    cells.insert(Location { x: 2, y: 0, z: 0 }, ACTIVE);
    cells.insert(Location { x: 2, y: 1, z: 0 }, ACTIVE);
    cells.insert(Location { x: 2, y: 2, z: 0 }, ACTIVE);

    assert_eq!(
        get_new_state(&cells, &Location { x: 0, y: 0, z: 0 }),
        INACTIVE
    );
    assert_eq!(
        get_new_state(&cells, &Location { x: 1, y: 0, z: 0 }),
        ACTIVE
    );
    assert_eq!(
        get_new_state(&cells, &Location { x: 1, y: 0, z: -1 }),
        ACTIVE
    );
    assert_eq!(
        get_new_state(&cells, &Location { x: 1, y: 0, z: 1 }),
        ACTIVE
    );
}

fn run_boot_cycle(cells: &HashMap<Location, State>) -> HashMap<Location, State> {
    let mut new_state = HashMap::new();

    // Since cells can only be affected by existing active cells, we'll use the input as starting
    // points. From those cells, we'll also check if neighbours need to be activated.
    for location in cells.keys() {
        for cell in get_surrounding_locations(&location) {
            if new_state.contains_key(&cell) {
                // Since we may visit the same cell multiple times, optimize a bit here.
                continue;
            }

            new_state.insert(cell, get_new_state(cells, &cell));
        }
    }

    return new_state;
}

#[test]
fn test_run_boot_cycle() {
    fn print_layer(cells: &HashMap<Location, State>, z: i32) -> String {
        let locations = cells.keys().filter(|l| l.z.eq(&z)).collect::<Vec<_>>();
        let min_x = locations.iter().map(|l| l.x).min().unwrap_or(0);
        let max_x = locations.iter().map(|l| l.x).max().unwrap_or(0);
        let min_y = locations.iter().map(|l| l.y).min().unwrap_or(0);
        let max_y = locations.iter().map(|l| l.y).max().unwrap_or(0);

        let mut result = "".to_owned();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match cells.get(&Location { x, y, z }) {
                    Some(ACTIVE) => result += "#",
                    _ => result += ".",
                }
            }
            result += "\n"
        }

        result.trim().to_owned()
    }

    let initial = parse_map(".#.\n..#\n###".to_owned(), 0);

    let cycle1 = run_boot_cycle(&initial);
    assert_eq!(
        print_layer(&cycle1, -1),
        ".....\n.....\n.#...\n...#.\n..#..".to_owned()
    );
    assert_eq!(
        print_layer(&cycle1, 0),
        ".....\n.....\n.#.#.\n..##.\n..#..".to_owned()
    );
    assert_eq!(
        print_layer(&cycle1, 1),
        ".....\n.....\n.#...\n...#.\n..#..".to_owned()
    );
}

pub fn puzzle1() {
    // Load data and run 6 boot cycles. The result is the number of active cells afterwards.
    let mut cells = match create_initial_state() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    for _ in 0..6 {
        cells = run_boot_cycle(&cells);
    }

    let active_cells: i32 = cells
        .values()
        .map(|s| match s {
            ACTIVE => 1,
            INACTIVE => 0,
        })
        .sum();
    println!(
        "Puzzle 1: There are {} cells active after 6 cycles.",
        active_cells
    );
}

// Copy a lot of things, want to keep puzzle 1, and puzzle 2 is not really data-compatible.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Location4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

fn create_initial_state4d() -> Result<HashMap<Location4d, State>, String> {
    let data = read_raw_input(17)?;
    Ok(parse_map4d(data))
}

fn parse_map4d(data: String) -> HashMap<Location4d, State> {
    let mut state = HashMap::new();
    let lines = data.split("\n").map(str::to_owned).collect::<Vec<_>>();
    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            match line.chars().nth(x) {
                Some('.') => {
                    state.insert(
                        Location4d {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        },
                        INACTIVE,
                    );
                }
                Some('#') => {
                    state.insert(
                        Location4d {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        },
                        ACTIVE,
                    );
                }
                _ => { /* ignore */ }
            }
        }
    }
    state
}

fn get_surrounding_locations4d(cell: &Location4d) -> Vec<Location4d> {
    let mut result = vec![];
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if w == 0 && z == 0 && y == 0 && x == 0 {
                        continue;
                    }
                    result.push(Location4d {
                        x: cell.x + x,
                        y: cell.y + y,
                        z: cell.z + z,
                        w: cell.w + w,
                    })
                }
            }
        }
    }
    result
}

fn get_new_state4d(cells: &HashMap<Location4d, State>, cell: &Location4d) -> State {
    // We'll need to consider all (26) direct neighbours (also diagonally)
    // The following applies:
    // If cell is active => remains active iff 2 or 3 neighbours are active
    // If cell is inactive => becomes active iff 3 neighbours are active

    let initial_state = match cells.get(cell) {
        Some(state) => *state,
        None => INACTIVE,
    };

    let active_cells_around: i32 = get_surrounding_locations4d(cell)
        .iter()
        .map(|l| match cells.get(l) {
            Some(ACTIVE) => 1,
            _ => 0,
        })
        .sum();

    match initial_state {
        ACTIVE if active_cells_around == 2 || active_cells_around == 3 => ACTIVE,
        INACTIVE if active_cells_around == 3 => ACTIVE,
        _ => INACTIVE,
    }
}

fn run_boot_cycle4d(cells: &HashMap<Location4d, State>) -> HashMap<Location4d, State> {
    let mut new_state = HashMap::new();

    // Since cells can only be affected by existing active cells, we'll use the input as starting
    // points. From those cells, we'll also check if neighbours need to be activated.
    for location in cells.keys() {
        for cell in get_surrounding_locations4d(&location) {
            if new_state.contains_key(&cell) {
                // Since we may visit the same cell multiple times, optimize a bit here.
                continue;
            }

            new_state.insert(cell, get_new_state4d(cells, &cell));
        }
    }

    return new_state;
}

pub fn puzzle2() {
    let mut cells = match create_initial_state4d() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    for _ in 0..6 {
        cells = run_boot_cycle4d(&cells);
    }

    let active_cells: i32 = cells
        .values()
        .map(|s| match s {
            ACTIVE => 1,
            INACTIVE => 0,
        })
        .sum();
    println!(
        "Puzzle 2: There are {} cells active after 6 cycles.",
        active_cells
    );
}
