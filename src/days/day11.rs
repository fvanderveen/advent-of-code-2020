use crate::days::day11::Cell::{Floor, Seat};
use crate::util::input::read_mapped_input;
use std::cmp::min;
use std::io::{stdout, Write};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Cell {
    Floor,
    Seat(bool),
}

fn parse_char(c: char) -> Result<Cell, String> {
    match c {
        '.' => Ok(Cell::Floor),
        'L' => Ok(Cell::Seat(false)),
        '#' => Ok(Cell::Seat(true)),
        _ => Err(format!("Invalid character in input: {}", c)),
    }
}

fn parse_line(line: String) -> Result<Vec<Cell>, String> {
    line.chars().map(parse_char).collect()
}

fn read_input() -> Result<Vec<Vec<Cell>>, String> {
    read_mapped_input(11, parse_line)
}

// This is basically a game of life. Floor tiles won't ever change, but chairs do; according to
// there two rules:
// 1 - If a seat is empty, and so are all seats adjacent to it (in all 8 directions) => it becomes occupied
// 2 - If a seat is occupied, and at least 4 other seats around it are as well => it becomes empty

fn get_surrounding_seats(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> Vec<&Cell> {
    let mut result = vec![];

    for ry in if y > 0 { y - 1 } else { 0 }..=min(y + 1, grid.len() - 1) {
        for rx in if x > 0 { x - 1 } else { 0 }..=min(x + 1, grid[0].len() - 1) {
            if rx == x && ry == y {
                continue;
            }

            result.push(&grid[ry][rx])
        }
    }

    result
}

fn get_visible_seat(
    grid: &Vec<Vec<Cell>>,
    (start_x, start_y): (usize, usize),
    (dx, dy): (i32, i32),
) -> Option<&Cell> {
    // Safe case to i32, should fit; given the puzzle input.
    let mut x = start_x as i32;
    let mut y = start_y as i32;

    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;

    loop {
        x += dx;
        y += dy;

        if x < 0 || y < 0 || x >= max_x || y >= max_y {
            return None;
        }

        match grid.get(y as usize).and_then(|r| r.get(x as usize)) {
            Some(cell @ Seat(_)) => return Some(cell),
            _ => continue,
        }
    }
}

fn get_visible_seats(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> Vec<&Cell> {
    let mut result = vec![];

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip useless case
            }
            if let Some(c) = get_visible_seat(grid, (x, y), (dx, dy)) {
                result.push(c);
            }
        }
    }

    result
}

// Run a single 'game of life' cycle, returns true if at least one cell changed state.
fn run_gol_cycle(
    grid: &mut Vec<Vec<Cell>>,
    get_occupied: fn(&Vec<Vec<Cell>>, usize, usize) -> usize,
    threshold: usize,
) -> bool {
    let input = grid.into_iter().map(|l| l.clone()).collect::<Vec<_>>();
    let mut result = false;

    for y in 0..input.len() {
        let row = &input[y];
        for x in 0..row.len() {
            match &row[x] {
                Floor => continue,
                Seat(true) => {
                    // Occupied seat, check if it needs to become empty
                    if get_occupied(&input, x, y) >= threshold {
                        grid[y][x] = Seat(false);
                        result = true;
                    }
                }
                Seat(false) => {
                    // Empty seat, check if it needs to become occupied
                    if get_occupied(&input, x, y) == 0 {
                        grid[y][x] = Seat(true);
                        result = true;
                    }
                }
            }
        }
    }

    result
}

fn _debug_print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        let line: String = row
            .iter()
            .map(|c| match c {
                Floor => '.',
                Seat(true) => '#',
                Seat(false) => 'L',
            })
            .collect();
        println!("{}", line);
    }
    println!();

    if let Err(e) = stdout().flush() {
        eprintln!("{}", e);
    }
}

#[test]
fn test_run_gol_cycle() {
    let mut input = vec![
        parse_line("L.LL.LL.LL".to_owned()).unwrap(),
        parse_line("LLLLLLL.LL".to_owned()).unwrap(),
        parse_line("L.L.L..L..".to_owned()).unwrap(),
        parse_line("LLLL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LLLLL.LL".to_owned()).unwrap(),
        parse_line("..L.L.....".to_owned()).unwrap(),
        parse_line("LLLLLLLLLL".to_owned()).unwrap(),
        parse_line("L.LLLLLL.L".to_owned()).unwrap(),
        parse_line("L.LLLLL.LL".to_owned()).unwrap(),
    ];

    let cycle1 = vec![
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#######.##".to_owned()).unwrap(),
        parse_line("#.#.#..#..".to_owned()).unwrap(),
        parse_line("####.##.##".to_owned()).unwrap(),
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
        parse_line("..#.#.....".to_owned()).unwrap(),
        parse_line("##########".to_owned()).unwrap(),
        parse_line("#.######.#".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
    ];

    let cycle2 = vec![
        parse_line("#.LL.L#.##".to_owned()).unwrap(),
        parse_line("#LLLLLL.L#".to_owned()).unwrap(),
        parse_line("L.L.L..L..".to_owned()).unwrap(),
        parse_line("#LLL.LL.L#".to_owned()).unwrap(),
        parse_line("#.LL.LL.LL".to_owned()).unwrap(),
        parse_line("#.LLLL#.##".to_owned()).unwrap(),
        parse_line("..L.L.....".to_owned()).unwrap(),
        parse_line("#LLLLLLLL#".to_owned()).unwrap(),
        parse_line("#.LLLLLL.L".to_owned()).unwrap(),
        parse_line("#.#LLLL.##".to_owned()).unwrap(),
    ];

    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        true
    );
    assert_eq!(input, cycle1);
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        true
    );
    assert_eq!(input, cycle2);
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle1, 4),
        false
    );
}

fn get_occupied_around_puzzle1(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> usize {
    get_surrounding_seats(grid, x, y)
        .into_iter()
        .filter(|c| match c {
            Seat(true) => true,
            _ => false,
        })
        .count()
}

pub fn puzzle1() {
    let mut grid = match read_input() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // Run simulation until stable:
    while run_gol_cycle(&mut grid, get_occupied_around_puzzle1, 4) {}

    // Count number of occupied seats:
    let occupied_seats = grid
        .into_iter()
        .flat_map(|row| row)
        .filter(|c| match c {
            Seat(true) => true,
            _ => false,
        })
        .count();
    println!("Puzzle 1: There are {} occupied seats", occupied_seats);
}

#[test]
fn test_run_gol_cycle2() {
    let mut input = vec![
        parse_line("L.LL.LL.LL".to_owned()).unwrap(),
        parse_line("LLLLLLL.LL".to_owned()).unwrap(),
        parse_line("L.L.L..L..".to_owned()).unwrap(),
        parse_line("LLLL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LLLLL.LL".to_owned()).unwrap(),
        parse_line("..L.L.....".to_owned()).unwrap(),
        parse_line("LLLLLLLLLL".to_owned()).unwrap(),
        parse_line("L.LLLLLL.L".to_owned()).unwrap(),
        parse_line("L.LLLLL.LL".to_owned()).unwrap(),
    ];

    let cycle1 = vec![
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#######.##".to_owned()).unwrap(),
        parse_line("#.#.#..#..".to_owned()).unwrap(),
        parse_line("####.##.##".to_owned()).unwrap(),
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
        parse_line("..#.#.....".to_owned()).unwrap(),
        parse_line("##########".to_owned()).unwrap(),
        parse_line("#.######.#".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
    ];

    let cycle2 = vec![
        parse_line("#.LL.LL.L#".to_owned()).unwrap(),
        parse_line("#LLLLLL.LL".to_owned()).unwrap(),
        parse_line("L.L.L..L..".to_owned()).unwrap(),
        parse_line("LLLL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LL.LL.LL".to_owned()).unwrap(),
        parse_line("L.LLLLL.LL".to_owned()).unwrap(),
        parse_line("..L.L.....".to_owned()).unwrap(),
        parse_line("LLLLLLLLL#".to_owned()).unwrap(),
        parse_line("#.LLLLLL.L".to_owned()).unwrap(),
        parse_line("#.LLLLL.L#".to_owned()).unwrap(),
    ];

    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    assert_eq!(input, cycle1);
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    _debug_print_grid(&input);
    assert_eq!(input, cycle2);
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        true
    );
    assert_eq!(
        run_gol_cycle(&mut input, get_occupied_around_puzzle2, 5),
        false
    );
}

#[test]
fn test_get_occupied_around_puzzle2() {
    let state = vec![
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#######.##".to_owned()).unwrap(),
        parse_line("#.#.#..#..".to_owned()).unwrap(),
        parse_line("####.##.##".to_owned()).unwrap(),
        parse_line("#.##.##.##".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
        parse_line("..#.#.....".to_owned()).unwrap(),
        parse_line("##########".to_owned()).unwrap(),
        parse_line("#.######.#".to_owned()).unwrap(),
        parse_line("#.#####.##".to_owned()).unwrap(),
    ];

    assert_eq!(get_occupied_around_puzzle2(&state, 0, 0), 3);
}

fn get_occupied_around_puzzle2(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> usize {
    get_visible_seats(grid, x, y)
        .into_iter()
        .filter(|c| match c {
            Seat(true) => true,
            _ => false,
        })
        .count()
}

pub fn puzzle2() {
    let mut grid = match read_input() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // Run simulation until stable:
    while run_gol_cycle(&mut grid, get_occupied_around_puzzle2, 5) {}

    // Count number of occupied seats:
    let occupied_seats = grid
        .into_iter()
        .flat_map(|row| row)
        .filter(|c| match c {
            Seat(true) => true,
            _ => false,
        })
        .count();
    println!("Puzzle 2: There are {} occupied seats", occupied_seats);
}
