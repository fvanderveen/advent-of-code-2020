// Tiles are in a hex grid
// Instructions are one of six directions: ne e se sw w ne
// Lines are a set of instructions without delimiters

// Grid stuff:
/*

/\/\/\
||||||
\/\/\/
||||||
/\/\/\

Start  0, 0
e  => +1,+0 // right on same line
w  => -1,+0 // left on same line
ne => +0,-1 // one line up, consider the x=0 to be at ne and sw
nw => +1,-1 // one line up, then right (ne => w)
se => -1,+1 // one line down, then left (sw => e)
sw => +0,+1 // one line down

start => nw => sw = w?
0,0 => 1,-1 => 1,0

 */

use crate::days::day24::Tile::{Black, White};
use crate::util::input::read_raw_input;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn translate(&self, dx: i32, dy: i32) -> Location {
        Location {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn to_location(line: &str) -> Result<Location, String> {
    let mut iter = line.chars();
    let mut x = 0;
    let mut y = 0;

    loop {
        match iter.next() {
            None => break, // Done!
            Some('e') => {
                // Move location east
                x -= 1;
            }
            Some('w') => {
                // Move location west
                x += 1;
            }
            Some('n') => {
                // We need another character (e or w)
                match iter.next() {
                    Some('e') => {
                        // Move north-east
                        y -= 1;
                    }
                    Some('w') => {
                        // Move north-west
                        y -= 1;
                        x += 1;
                    }
                    Some(v) => {
                        return Err(format!(
                            "Unexpected character '{}', expected either a 'e' or 'w' after 'n'. {}",
                            v, line
                        ))
                    }
                    None => {
                        return Err(format!(
                            "Unexpected EOL, expected 'e' or 'w' after 'n', {}",
                            line
                        ))
                    }
                }
            }
            Some('s') => {
                // We need another character (e or w)
                match iter.next() {
                    Some('e') => {
                        // Move south-east
                        y += 1;
                        x -= 1;
                    }
                    Some('w') => {
                        // Move south-west
                        y += 1;
                    }
                    Some(v) => {
                        return Err(format!(
                            "Unexpected character '{}', expected either a 'e' or 'w' after 'n'. {}",
                            v, line
                        ))
                    }
                    None => {
                        return Err(format!(
                            "Unexpected EOL, expected 'e' or 'w' after 'n', {}",
                            line
                        ))
                    }
                }
            }
            Some(v) => return Err(format!("Invalid character in line: '{}' ({})", v, line)),
        }
    }

    Ok(Location { x, y })
}

#[test]
fn test_to_location() {
    assert_eq!(to_location("ew"), Ok(Location { x: 0, y: 0 }));
    assert_eq!(to_location("se"), Ok(Location { x: -1, y: 1 }));
    assert_eq!(to_location("sw"), Ok(Location { x: 0, y: 1 }));
    assert_eq!(to_location("esew"), Ok(Location { x: -1, y: 1 }));
    assert_eq!(to_location("nwwswee"), Ok(Location { x: 0, y: 0 }));
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Tile {
    Black,
    White,
}

pub fn puzzle1() {
    let locations = match read_raw_input(24).and_then(|d| {
        d.split("\n")
            .map(to_location)
            .collect::<Result<Vec<_>, _>>()
    }) {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    let mut tiles = HashMap::new();

    for location in locations {
        // Each line represents a location, see above.
        match tiles.get_mut(&location) {
            None => {
                tiles.insert(location, Black);
            }
            Some(p) => *p = if Black.eq(p) { White } else { Black },
        }
    }

    let number_of_black_tiles = tiles.values().filter(|&t| Black.eq(t)).count();
    println!(
        "Puzzle 1: After flipping all tiles, we have {} black tiles",
        number_of_black_tiles
    );

    // Puzzle 2 is here as it continues on the map we built.
    let mut result = tiles;
    for _i in 0..100 {
        result = run_game_of_life_loop(&result);
    }

    let number_of_black_tiles_p2 = result.values().filter(|&t| Black.eq(t)).count();
    println!(
        "Puzzle 2: After running 100 days of GoL, we have {} black tiles",
        number_of_black_tiles_p2
    );
}

fn get_locations_around(location: &Location) -> Vec<Location> {
    // Each tile touches the tiles in all six directions:
    // e  => +1,+0 // right on same line
    // w  => -1,+0 // left on same line
    // ne => +0,-1 // one line up, consider the x=0 to be at ne and sw
    // nw => +1,-1 // one line up, then right (ne => w)
    // se => -1,+1 // one line down, then left (sw => e)
    // sw => +0,+1 // one line down
    return vec![
        location.translate(1, 0),  // e
        location.translate(-1, 0), // w
        location.translate(0, -1), // ne
        location.translate(1, -1), // nw
        location.translate(-1, 1), // se
        location.translate(0, 1),  // sw
    ];
}

fn get_tiles_around(location: &Location, map: &HashMap<Location, Tile>) -> Vec<Tile> {
    return get_locations_around(location)
        .iter()
        .map(|l| map.get(&l).map(|t| t.clone()).unwrap_or(White))
        .collect();
}

fn run_game_of_life_loop(input: &HashMap<Location, Tile>) -> HashMap<Location, Tile> {
    let mut result = HashMap::new();

    let locations: HashSet<Location, RandomState> = HashSet::from_iter(
        input
            .keys()
            .map(Location::clone)
            .chain(input.keys().flat_map(|l| get_locations_around(l))),
    );

    for location in locations {
        let black_tiles = get_tiles_around(&location, input)
            .iter()
            .filter(|t| Black.eq(t))
            .count();

        // GoL rules:
        // - if black, it only stays black if exactly 1 or 2 touching tiles are black.
        // - if white, it becomes black if exactly 2 touching tiles are black.
        match input.get(&location).unwrap_or(&White) {
            Black => result.insert(
                location,
                if black_tiles == 1 || black_tiles == 2 {
                    Black
                } else {
                    White
                },
            ),
            White => result.insert(location, if black_tiles == 2 { Black } else { White }),
        };
    }

    result
}

pub fn puzzle2() {}
