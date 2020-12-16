use crate::util::input::read_raw_input;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Field {
    name: String,
    validity: Vec<Range<u128>>,
}

fn parse_range(data: &str) -> Result<Range<u128>, String> {
    let parts = data
        .split("-")
        .map(str::trim)
        .map(|s| s.parse::<u128>().map_err(|e| format!("{} ({})", e, s)))
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 2 {
        return Err(format!(
            "Expected two parts for the range, but got {} in {}",
            parts.len(),
            data
        ));
    }

    let lower = parts[0];
    let upper = parts[1] + 1; // upper is inclusive, range is exclusive

    return Ok(lower..upper);
}

fn parse_input() -> Result<(Vec<Field>, Vec<u128>, Vec<Vec<u128>>), String> {
    let lines = read_raw_input(16).map(|d| d.split("\n").map(str::to_owned).collect::<Vec<_>>())?;
    let mut i = 0;

    let mut fields = vec![];
    loop {
        // We read lines as fields until we encounter a blank one.
        let line = &lines[i];
        i += 1;

        if line.len() == 0 {
            break;
        }

        // Name is the line until the ':'
        let name = line.chars().take_while(|c| c.ne(&':')).collect::<String>();
        // The rest needs to be split on " or ", then "-" and parsed to a range
        let validity = line
            .chars()
            .skip_while(|c| c.ne(&':'))
            .skip(1)
            .collect::<String>()
            .split(" or ")
            .map(parse_range)
            .collect::<Result<Vec<_>, _>>()?;

        fields.push(Field { name, validity });
    }

    // expect next line to be "your ticket:", so we're skipping it.
    i += 1;
    let own_ticket = lines[i]
        .split(",")
        .map(|v| v.parse::<u128>().map_err(|e| format!("{} ({})", e, v)))
        .collect::<Result<Vec<_>, _>>()?;

    // Next lines are a blank and the "other tickets:", so we're also skipping that. (+1 to move past the own ticket line we just parsed)
    i += 3;

    let other_tickets = lines[i..]
        .iter()
        .map(|l| {
            l.split(",")
                .map(|v| v.parse::<u128>().map_err(|e| format!("{} ({})", e, v)))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((fields, own_ticket, other_tickets))
}

fn is_valid_for_any_field(value: &u128, fields: &Vec<Field>) -> bool {
    for field in fields {
        if is_valid_for_field(value, field) {
            return true;
        }
    }

    return false;
}

fn is_valid_for_field(value: &u128, field: &Field) -> bool {
    field.validity.iter().any(|r| r.contains(value))
}

pub fn puzzle1() {
    let (fields, _, other_tickets) = match parse_input() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // We need to find fields in the other_tickets that are not fitting _any_ of the valid field ranges.
    // The result of this puzzle is the sum of such fields.

    let result = other_tickets
        .iter()
        .map(|v| {
            v.iter()
                .filter(|val| !is_valid_for_any_field(val, &fields))
                .sum::<u128>()
        })
        .sum::<u128>();
    println!(
        "Puzzle 1: sum of fields not fitting any validity range = {}",
        result
    );
}
pub fn puzzle2() {
    let (fields, my_ticket, other_tickets) = match parse_input() {
        Err(e) => {
            return eprintln!("{}", e);
        }
        Ok(v) => v,
    };

    // First, remove invalid tickets from the other_tickets list
    let valid_tickets = other_tickets
        .into_iter()
        .filter(|t| t.iter().all(|v| is_valid_for_any_field(v, &fields)))
        .collect::<Vec<_>>();

    // Then, figure out which field is at which index.
    // 1: Gather which indexes fit the fields.
    // 2: Try finding unique ones:
    // 2a: Get the ones with one index, and assign those
    // 2b: Remove those indexes from the other fields, and continue

    let mut fixed_indexes = vec![];
    let mut indexed_fields: HashMap<&Field, usize> = HashMap::new();

    // DEBUG: Print all fields with applicable indexes:
    for field in &fields {
        let mut indexes = vec![];
        let mut indexes_with_own = vec![];

        for i in 0..fields.len() {
            if valid_tickets
                .iter()
                .all(|v| is_valid_for_field(&v[i], field))
            {
                indexes.push(i);
                if is_valid_for_field(&my_ticket[i], field) {
                    indexes_with_own.push(i);
                }
            }
        }

        println!(
            "DEBUG: {:?} => {:?} / {:?}",
            field, indexes, indexes_with_own
        )
    }

    loop {
        let todo = fields
            .iter()
            .filter(|f| match indexed_fields.get(f) {
                Some(_) => false,
                None => true,
            })
            .collect::<Vec<_>>();

        if todo.len() == 0 {
            break;
        }

        let mut assigned = false;

        for field in todo {
            let mut indexes = vec![];

            for i in 0..fields.len() {
                if fixed_indexes.contains(&i) {
                    continue;
                }

                if valid_tickets
                    .iter()
                    .all(|v| is_valid_for_field(&v[i], field))
                    && is_valid_for_field(&my_ticket[i], field)
                {
                    indexes.push(i);
                }
            }

            if indexes.len() == 1 {
                let field_index = indexes[0];
                println!("Fixed field {:?} to index {}", field, field_index);
                fixed_indexes.push(field_index);
                indexed_fields.insert(field, field_index);

                assigned = true;
            }
        }

        if !assigned {
            panic!("Could not assign any fields this loop!");
        }
    }

    println!("Assigned all fields: {:?}", indexed_fields);

    // Finally, the answer to our puzzle is the multiplication of all 'departure *' fields of our ticket
    let departure_fields = fields
        .iter()
        .filter(|f| f.name.starts_with("departure "))
        .collect::<Vec<_>>();
    if departure_fields.len() != 6 {
        panic!(
            "Expected 6 departure fields per puzzle, but got {} => {:?}",
            departure_fields.len(),
            departure_fields
        );
    }

    let mut result = 1;
    for field in departure_fields {
        if let Some(index) = indexed_fields.get(field) {
            result *= my_ticket[*index];
        } else {
            panic!(
                "Why is this field not indexed?! {:?} in {:?}",
                field, indexed_fields
            );
        }
    }

    println!("Puzzle 2: Result = {}", result);
}
