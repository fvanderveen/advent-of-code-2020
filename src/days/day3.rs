use std::fs::read_to_string;

#[derive(Eq, PartialEq)]
enum Cell {
    Empty,
    Tree,
}

fn parse_char(chr: char) -> Result<Cell, String> {
    match chr {
        '#' => Ok(Cell::Tree),
        '.' => Ok(Cell::Empty),
        _ => Err(format!("Invalid character '{}' in data", chr)),
    }
}

fn parse_line(line: &str) -> Result<Vec<Cell>, String> {
    line.chars().map(parse_char).collect()
}

fn read_input_file() -> Result<Vec<Vec<Cell>>, String> {
    let data = read_to_string("input/day3.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            let results: Result<Vec<_>, _> = data.split("\n").map(parse_line).collect();
            match results {
                Err(details) => Err(format!("Could not parse all lines: {}", details)),
                Ok(entries) => Ok(entries),
            }
        }
    };
}

fn get_tree_count(map: &Vec<Vec<Cell>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() {
        if map[y][x] == Cell::Tree {
            trees += 1;
        }

        x += dx;
        x %= map[y].len(); // wrap around
        y += dy;
    }

    trees
}

pub fn puzzle1() {
    let cells = match read_input_file() {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(cells) => cells,
    };

    // We need to start top-left (0,0)
    // Every time we make a move of 3 right, 1 down
    // Count the number of tree-cells we find
    println!("Puzzle 1: found {} trees", get_tree_count(&cells, 3, 1));
}

pub fn puzzle2() {
    let cells = match read_input_file() {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(cells) => cells,
    };

    // We need to find the trees encountered on the following slopes:
    // dx|dy
    //  1| 1
    //  3| 1
    //  5| 1
    //  7| 1
    //  1| 2
    // And multiply the results for the answer.
    let tree11 = get_tree_count(&cells, 1, 1);
    let tree31 = get_tree_count(&cells, 3, 1);
    let tree51 = get_tree_count(&cells, 5, 1);
    let tree71 = get_tree_count(&cells, 7, 1);
    let tree12 = get_tree_count(&cells, 1, 2);

    println!("Puzzle 2:");
    println!("Right 1, down 1 = {}", tree11);
    println!("Right 3, down 1 = {}", tree31);
    println!("Right 5, down 1 = {}", tree51);
    println!("Right 7, down 1 = {}", tree71);
    println!("Right 1, down 2 = {}", tree12);
    println!(
        "Puzzle answer = {}",
        tree11 * tree31 * tree51 * tree71 * tree12
    );
}
