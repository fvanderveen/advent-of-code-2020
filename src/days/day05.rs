use crate::util::input::read_mapped_input;
use core::fmt;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seat#{} (row {}, col {})", self.id, self.row, self.col)
    }
}

fn parse_seat(location: String) -> Result<Seat, String> {
    // The location is binary-encoded for a seat in one of the 128 rows, and 8 columns.
    // The row is encoded as a string of 7 'F' or 'B' characters, where F means lower and B means upper half.
    // Same for col, encoded using 3 'L' or 'R' characters, where L means lower and R means upper half.
    let mut min_row = 0;
    let mut max_row = 128; // Exclusive, but easier to divide by two

    for i in 0..7 {
        match location.chars().nth(i) {
            Some('F') => max_row -= (max_row - min_row) / 2,
            Some('B') => min_row += (max_row - min_row) / 2,
            _ => continue, // Ignore unknowns
        }
    }

    let mut min_col = 0;
    let mut max_col = 8; // Exclusive

    for i in 7..10 {
        match location.chars().nth(i) {
            Some('L') => max_col -= (max_col - min_col) / 2,
            Some('R') => min_col += (max_col - min_col) / 2,
            _ => continue,
        }
    }

    Ok(Seat {
        row: min_row,
        col: min_col,
        id: min_row * 8 + min_col,
    })
}

#[test]
fn test_parse_seat() {
    assert_eq!(
        parse_seat("FBFBBFFRLR".to_owned()),
        Ok(Seat {
            row: 44,
            col: 5,
            id: 357
        })
    );
    assert_eq!(
        parse_seat("BFFFBBFRRR".to_owned()),
        Ok(Seat {
            row: 70,
            col: 7,
            id: 567
        })
    );
    assert_eq!(
        parse_seat("FFFBBBFRRR".to_owned()),
        Ok(Seat {
            row: 14,
            col: 7,
            id: 119
        })
    );
    assert_eq!(
        parse_seat("BBFFBBFRLL".to_owned()),
        Ok(Seat {
            row: 102,
            col: 4,
            id: 820
        })
    );
}

pub fn puzzle1() {
    let seats = match read_mapped_input(5, parse_seat) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let max_seat_by_id = seats.iter().max_by(|s1, s2| s1.id.cmp(&s2.id)).unwrap();
    println!("Puzzle 1, max seat ID = {}", max_seat_by_id.id);
}

fn get_free_seats(seats: &HashMap<i32, Seat>) -> Vec<Seat> {
    let mut result = vec![];

    for row in 0..128 {
        for col in 0..8 {
            let id = row * 8 + col;
            match seats.get(&id) {
                None => result.push(Seat { row, col, id }),
                _ => continue,
            }
        }
    }

    result
}

pub fn puzzle2() {
    let seats = match read_mapped_input(5, parse_seat) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => HashMap::from_iter(v.into_iter().map(|s| (s.id, s))),
    };

    // Find free seat in the plane, some at the front & end do not exist, so those shouldn't be our
    // seat. (To check, the seats with ID n-1 and n+1 (n = your seat) should exist)

    let free_seats = get_free_seats(&seats);
    let mut existing_free_seats = vec![];

    for seat in free_seats {
        match (seats.get(&(seat.id - 1)), seats.get(&(seat.id + 1))) {
            (Some(_), Some(_)) => existing_free_seats.push(seat),
            _ => continue,
        }
    }

    match existing_free_seats.len() {
        1 => println!("Puzzle 2: Found > {}", existing_free_seats[0]),
        _ => println!(
            "Puzzle 2: ERROR! Did not find exactly one free seat: {}",
            existing_free_seats
                .into_iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}
