use crate::days::day20::Pixel::{Black, White};
use crate::days::day20::Side::{East, North, South, West};
use crate::util::input::read_raw_input;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Pixel {
    Black,
    White,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Black => write!(f, "."),
            White => write!(f, "#"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Tile {
    id: u32,
    image: [[Pixel; 10]; 10],
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {}:\n{}", self.id, image_to_string(self.image))
    }
}

fn image_to_string(image: [[Pixel; 10]; 10]) -> String {
    image
        .iter()
        .map(|l| l.iter().map(|p| p.to_string()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_tile(data: &str) -> Result<Tile, String> {
    let lines = data
        .trim()
        .split("\n")
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if lines.len() != 11 {
        return Err(format!(
            "Expected tile data of 11 lines, but got {}",
            lines.len()
        ));
    }

    let tile_id = lines[0]
        .chars()
        .skip_while(|c| !c.is_digit(10))
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .map_err(|e| format!("Could not parse tile ID from {}: {}", lines[0], e))?;

    let pixels = lines[1..]
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Ok(White),
                    '.' => Ok(Black),
                    _ => Err(format!("Invalid pixel char '{}'", c)),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let pixel_map: [[Pixel; 10]; 10] = pixels
        .iter()
        .map(|l| {
            let array: [Pixel; 10] = l[0..10].try_into().unwrap();
            array
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Ok(Tile {
        id: tile_id,
        image: pixel_map,
    })
}

fn read_tiles() -> Result<Vec<Tile>, String> {
    read_raw_input(20).and_then(|d| {
        d.split("\n\n")
            .map(parse_tile)
            .collect::<Result<Vec<_>, _>>()
    })
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Side {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            North => write!(f, "North"),
            East => write!(f, "East"),
            South => write!(f, "South"),
            West => write!(f, "West"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

fn get_side(tile: &Tile, side: Side) -> Vec<Pixel> {
    match side {
        North => tile.image[0].to_vec(),
        South => tile.image[9].to_vec(),
        East => tile.image.iter().map(|l| l[9]).collect(),
        West => tile.image.iter().map(|l| l[0]).collect(),
    }
}

fn flip(tile: &Tile) -> Tile {
    let flipped: [[Pixel; 10]; 10] = tile
        .image
        .iter()
        .map(|l| -> [Pixel; 10] {
            l.iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    Tile {
        id: tile.id,
        image: flipped,
    }
}

fn rotate(tile: &Tile, orientation: Side) -> Tile {
    match orientation {
        North => tile.clone(), // Default orientation
        South => {
            let image: [[Pixel; 10]; 10] = tile
                .image
                .iter()
                .map(|l| -> [Pixel; 10] {
                    l.iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .rev()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Tile { id: tile.id, image }
        }
        West => {
            let mut image: Vec<[Pixel; 10]> = vec![];
            for i in 0..10 {
                image.push(
                    tile.image
                        .iter()
                        .map(|l| l[i])
                        .rev()
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );
            }
            Tile {
                id: tile.id,
                image: image.try_into().unwrap(),
            }
        }
        East => {
            let mut image: Vec<[Pixel; 10]> = vec![];
            for i in 0..10 {
                image.insert(
                    0,
                    tile.image
                        .iter()
                        .map(|l| l[i])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );
            }
            Tile {
                id: tile.id,
                image: image.try_into().unwrap(),
            }
        }
    }
}

fn fits(tile: Tile, location: Location, map: &HashMap<Location, Tile>) -> bool {
    let north = map
        .get(&location.translate(0, -1))
        .map(|t| get_side(t, South));
    let south = map
        .get(&location.translate(0, 1))
        .map(|t| get_side(t, North));
    let west = map
        .get(&location.translate(-1, 0))
        .map(|t| get_side(t, East));
    let east = map
        .get(&location.translate(1, 0))
        .map(|t| get_side(t, West));

    if north.is_none() && south.is_none() && west.is_none() && east.is_none() {
        return true;
    }

    let fits_north = north
        .clone()
        .map(|v| get_side(&tile, North).eq(&v))
        .unwrap_or(true);
    let fits_south = south
        .clone()
        .map(|v| get_side(&tile, South).eq(&v))
        .unwrap_or(true);
    let fits_east = east
        .clone()
        .map(|v| get_side(&tile, East).eq(&v))
        .unwrap_or(true);
    let fits_west = west
        .clone()
        .map(|v| get_side(&tile, West).eq(&v))
        .unwrap_or(true);

    return fits_north && fits_south && fits_west && fits_east;
}

fn brute_force(
    tiles: Vec<Tile>,
    map: &HashMap<Location, Tile>,
    location: Location,
    size: i32,
    level: u32,
) -> Option<(usize, HashMap<Location, Tile>)> {
    if tiles.len() == 0 {
        // We done it!
        // Get the IDs in the corners, and sum them
        let tl = map.get(&Location { x: 0, y: 0 }).unwrap().id as usize;
        let tr = map.get(&Location { x: size - 1, y: 0 }).unwrap().id as usize;
        let bl = map.get(&Location { x: 0, y: size - 1 }).unwrap().id as usize;
        let br = map
            .get(&Location {
                x: size - 1,
                y: size - 1,
            })
            .unwrap()
            .id as usize;
        return Some((tl * tr * bl * br, map.clone()));
    }

    // Find all tiles and orientations we can fit in `location`, and continue with that:
    for tile in &tiles {
        for orientation in vec![North, South, East, West] {
            for flipped in vec![true, false] {
                let rotated = if flipped {
                    rotate(&flip(&tile), orientation)
                } else {
                    rotate(&tile, orientation)
                };

                if fits(rotated, location, &map) {
                    // Create a clone of the map, and insert this tile
                    let mut sub_map = map.clone();
                    sub_map.insert(location, rotated);
                    let sub_tiles = tiles
                        .iter()
                        .filter(|t| t.id != tile.id)
                        .cloned()
                        .collect::<Vec<_>>();

                    // get a next locations:
                    let next_location = if location.x == (size - 1) {
                        Location {
                            x: 0,
                            y: location.y + 1,
                        }
                    } else {
                        Location {
                            x: location.x + 1,
                            y: location.y,
                        }
                    };

                    if let Some(result) =
                        brute_force(sub_tiles, &sub_map, next_location, size, level + 1)
                    {
                        return Some(result);
                    }
                }
            }
        }
    }

    None
}

#[allow(unreachable_code)]
pub fn puzzle1() {
    return; // Heavy, so ignore by default (we stored the map)

    // Input are image tiles. The borders should line up, we need to assemble the image
    let tiles = match read_tiles() {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    println!("Read info about {} tiles", tiles.len());

    let size = (tiles.len() as f64).sqrt() as i32;
    println!("Expecting to form a {0}x{0} grid", size);

    let map = HashMap::new();
    let (p1, map) = match brute_force(tiles, &map, Location { x: 0, y: 0 }, size, 0) {
        Some(v) => v,
        None => return println!("Puzzle 1: no result, probably a bug."),
    };

    println!("Puzzle 1: Result = {}", p1);

    // Puzzle 2 is here, due to the use of the map.

    // Create the full image by stripping the tile borders
    let mut image: Vec<String> = vec![];
    for y in 0..size {
        for l in 1..9 {
            let mut line = "".to_owned();
            for x in 0..size {
                line += map.get(&Location { x, y }).unwrap().image[l][1..9]
                    .iter()
                    .map(Pixel::to_string)
                    .collect::<String>()
                    .as_str();
            }
            image.push(line);
        }
    }

    println!("Map:");
    for line in &image {
        println!("{}", line);
    }
}

pub fn puzzle2() {
    let map = match read_to_string("input/day20_map.txt").map(|map| {
        map.trim()
            .split("\n")
            .map(str::to_owned)
            .collect::<Vec<_>>()
    }) {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    // Well; let's try with a pre-parsed map!

    // The following offsets represent where '#' should be to match a sea-monster
    let points = vec![
        Location { y: 0, x: 18 },
        Location { y: 1, x: 0 },
        Location { y: 1, x: 5 },
        Location { y: 1, x: 6 },
        Location { y: 1, x: 11 },
        Location { y: 1, x: 12 },
        Location { y: 1, x: 17 },
        Location { y: 1, x: 18 },
        Location { y: 1, x: 19 },
        Location { y: 2, x: 1 },
        Location { y: 2, x: 4 },
        Location { y: 2, x: 7 },
        Location { y: 2, x: 10 },
        Location { y: 2, x: 13 },
        Location { y: 2, x: 16 },
    ];

    let size = map.len() as i32;

    let mut mutated = vec![];

    // Found by trying, this orientation is the only one that finds monsters!
    for i in 0..size {
        mutated.push(
            map.iter()
                .map(|l| l.chars().nth(i as usize).unwrap())
                .collect::<String>(),
        );
    }

    let mut highlighted = mutated.clone();

    for y in 0..size {
        for x in 0..size {
            if points.iter().map(|l| l.translate(x, y)).all(|l| {
                match mutated
                    .get(l.y as usize)
                    .and_then(|r| r.chars().nth(l.x as usize))
                {
                    Some('#') => true,
                    _ => false,
                }
            }) {
                println!("Found a sea-monster at {},{}", x, y);

                points.iter().map(|l| l.translate(x, y)).for_each(|l| {
                    let line = highlighted.get(l.y as usize).unwrap();
                    highlighted[l.y as usize] = format!(
                        "{}{}{}",
                        &line[0..l.x as usize],
                        "@",
                        &line[l.x as usize + 1..]
                    );
                });
            }
        }
    }

    println!("Highlighted map:\n{}", highlighted.join("\n"));

    let result: usize = highlighted
        .iter()
        .map(|l| l.chars().filter(|c| c.eq(&'#')).count())
        .sum();
    println!("Puzzle 2: Non-monster rough tiles = {}", result);
}
