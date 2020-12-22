use crate::days::day20::Pixel::{Black, White};
use crate::days::day20::Side::{East, North, South, West};
use crate::util::input::read_raw_input;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

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
        write!(
            f,
            "Tile {}:\n{}",
            self.id,
            self.image
                .iter()
                .map(|l| l.iter().map(|p| p.to_string()).collect::<String>() + "\n")
                .collect::<String>()
        )
    }
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

fn reversed(pixels: Vec<Pixel>) -> Vec<Pixel> {
    pixels.into_iter().rev().collect()
}

fn get_side(tile: &Tile, side: Side) -> Vec<Pixel> {
    match side {
        North => tile.image[0].to_vec(),
        South => tile.image[9].iter().cloned().rev().collect(),
        East => tile.image.iter().map(|l| l[9]).collect(),
        West => tile.image.iter().map(|l| l[0]).rev().collect(),
    }
}

fn align(tile: &Tile, side: Side, target: &Tile) -> Vec<Side> {
    let mut search = get_side(tile, side);
    search.reverse(); // Reverse search, as we need to find sides that actually mirror it.

    let mut result = vec![];

    if search.eq(&get_side(target, North)) {
        result.push(North)
    }
    if search.eq(&get_side(target, East)) {
        result.push(East)
    }
    if search.eq(&get_side(target, South)) {
        result.push(South)
    }
    if search.eq(&get_side(target, West)) {
        result.push(West)
    }

    result
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

fn inverse(side: Side) -> Side {
    match side {
        North => South,
        South => North,
        West => East,
        East => West,
    }
}

fn orientation_for_align(side: Side, alignment: Side) -> Side {
    // Return a side that can be used for orientation, so that the to-be-aligned tile
    // is oriented correctly to be on the given side of a tile, using its alignment side.

    match side {
        North => inverse(alignment),
        South => alignment,
        East => match alignment {
            North => East,
            South => West,
            East => South,
            West => North,
        },
        West => match alignment {
            North => West,
            South => East,
            East => North,
            West => South,
        },
    }
}

fn try_fit_tile(
    point: &Location,
    grid: &HashMap<Location, Tile>,
    tiles: &Vec<&Tile>,
) -> Option<Tile> {
    // Check which sides have tiles, and which sides we need to match up.
    let north = grid
        .get(&point.translate(0, -1))
        .map(|t| reversed(get_side(t, South)));
    let south = grid
        .get(&point.translate(0, 1))
        .map(|t| reversed(get_side(t, North)));
    let west = grid
        .get(&point.translate(-1, 0))
        .map(|t| reversed(get_side(t, East)));
    let east = grid
        .get(&point.translate(1, 0))
        .map(|t| reversed(get_side(t, West)));

    if north.is_none() && south.is_none() && west.is_none() && east.is_none() {
        return None;
    }

    // For each tile, try all orientations, and see if it fits against the sides that are there.
    // If we find just one tile, that's the one. Otherwise, we can't be sure and return nothing.
    let mut options = vec![];
    for tile in tiles {
        for orientation in vec![North, South, East, West] {
            let rotated = rotate(tile, orientation);
            let fits_north = north
                .clone()
                .map(|v| get_side(&rotated, North).eq(&v))
                .unwrap_or(true);
            let fits_south = south
                .clone()
                .map(|v| get_side(&rotated, South).eq(&v))
                .unwrap_or(true);
            let fits_east = east
                .clone()
                .map(|v| get_side(&rotated, East).eq(&v))
                .unwrap_or(true);
            let fits_west = west
                .clone()
                .map(|v| get_side(&rotated, West).eq(&v))
                .unwrap_or(true);

            if fits_north && fits_south && fits_east && fits_west {
                options.push(rotated);
            }
        }
    }

    if options.len() == 1 {
        Some(options[0])
    } else {
        None
    }
}

pub fn puzzle1() {
    // Input are image tiles. The borders should line up, we need to assemble the image
    let tiles = match read_tiles() {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    println!("Read info about {} tiles", tiles.len());

    let size = (tiles.len() as f64).sqrt() as usize;
    println!("Expecting to form a {0}x{0} grid", size);

    let mut min_x = -1;
    let mut max_x = 1;
    let mut min_y = -1;
    let mut max_y = 1;
    let mut grid = HashMap::new();
    grid.insert(Location { x: 0, y: 0 }, tiles[0]);

    // Try to fill tile uniquely around already known tiles:
    loop {
        println!(
            "There are {} tiles left to place",
            tiles
                .iter()
                .filter(|t| !grid.values().any(|pt| pt.id == t.id))
                .count()
        );

        let mut placed = false;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                // If we don't have a tile here, just continue.
                let point = Location { x, y };
                if grid.get(&point).is_some() {
                    // Already filled, so let's continue
                    continue;
                }

                let available_tiles = tiles
                    .iter()
                    .filter(|t| !grid.values().any(|pt| pt.id == t.id))
                    .collect::<Vec<_>>();

                if let Some(tile) = try_fit_tile(&point, &grid, &available_tiles) {
                    // We matched a single option, let's set it.
                    println!("Found a tile for ({},{})", point.x, point.y);
                    grid.insert(point, tile);
                    placed = true;
                }
            }
        }

        if !placed {
            panic!("Couldn't place any tiles this round :(");
        }

        println!("Current image:");
        for y in min_y..=max_y {
            let mut line = grid.iter().filter(|(k, _)| k.y == y).collect::<Vec<_>>();
            line.sort_by(|(l1, _), (l2, _)| l1.x.cmp(&l2.x));
            for i in 0..10 {
                println!(
                    "| {} |",
                    line.iter()
                        .map(|(_, t)| t.image[i].iter().map(Pixel::to_string).collect::<String>())
                        .collect::<String>()
                );
            }
        }

        // Calculate new min and max values (grow the grid to fill by one slot)
        min_x = grid.keys().map(|l| l.x).min().unwrap_or(min_x);
        max_x = grid.keys().map(|l| l.x).max().unwrap_or(max_x);
        min_y = grid.keys().map(|l| l.y).min().unwrap_or(min_y);
        max_y = grid.keys().map(|l| l.y).max().unwrap_or(max_y);
    }
}

pub fn puzzle2() {}
