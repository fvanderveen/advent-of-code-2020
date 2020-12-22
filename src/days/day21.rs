use crate::util::input::read_mapped_input;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse_allergens(line: &str) -> Vec<String> {
    if line.len() == 0 {
        return vec![];
    }

    line["(contains ".len()..]
        .split(",")
        .map(str::trim)
        .map(str::to_owned)
        .collect()
}

fn parse_food(line: String) -> Result<Food, String> {
    let index = line.find("(").unwrap_or(line.len());
    let ingredients = line[0..index]
        .split(" ")
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .map(str::to_owned)
        .collect();
    let allergens = parse_allergens(line[index..line.len() - 1].trim());

    Ok(Food {
        ingredients,
        allergens,
    })
}

fn read_input() -> Result<Vec<Food>, String> {
    read_mapped_input(21, parse_food)
}

pub fn puzzle1() {
    let foods = match read_input() {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    let allergens: HashSet<String, RandomState> =
        HashSet::from_iter(foods.iter().flat_map(|f| f.allergens.clone()));
    let mut allergen_map: HashMap<String, String> = HashMap::new();

    // For each allergen, find all ingredients that contain it; and deduce the unknown language translation for it.
    // We loop here, in case we cannot deduce all uniquely in the first round
    loop {
        let allergens_to_map = &allergens
            .iter()
            .filter(|a| allergen_map.get(*a).is_none())
            .collect::<Vec<_>>();
        if allergens_to_map.len() == 0 {
            println!("Mapped all allergens to their translation!");
            break;
        }

        println!("Looking for {} allergens", allergens_to_map.len());

        let mut done_work = false;
        for allergen in allergens_to_map {
            let food_with_allergen = &foods
                .iter()
                .filter(|f| f.allergens.contains(&allergen))
                .collect::<Vec<_>>();

            let ingredients: HashSet<String, RandomState> = HashSet::from_iter(
                food_with_allergen
                    .iter()
                    .flat_map(|f| f.ingredients.clone()),
            );
            let options = ingredients
                .into_iter()
                .filter(|i| !allergen_map.values().any(|v| v.eq(i)))
                .filter(|i| food_with_allergen.iter().all(|f| f.ingredients.contains(i)))
                .collect::<Vec<_>>();

            if options.len() == 1 {
                allergen_map.insert(allergen.to_string(), options[0].to_string());
                done_work = true;
                println!("Mapped {} to {}", allergen, options[0]);
            } else {
                println!(
                    "Found {} options for {} ({:?})",
                    options.len(),
                    allergen,
                    options
                );
            }
        }

        if !done_work {
            panic!("Could not uniquely find ingredients any more, but we're also not done!");
        }
    }

    println!("Map: {:?}", allergen_map);

    // Now that we have a translation map, finish the puzzle!
    // We need to count all ingredients in the list that are _not_ an allergen:
    let result = foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| !allergen_map.values().any(|a| a.eq(*i)))
                .count()
        })
        .sum::<usize>();
    println!(
        "Puzzle 1: There are {} ingredient occurrences that can not be an allergen",
        result
    );

    // Sort allergens (map key) alphabetically, then map to values, and produce a comma-separated string
    let mut keys = allergen_map.keys().collect::<Vec<_>>();
    keys.sort();
    let result2 = keys
        .iter()
        .map(|k| allergen_map.get(*k).unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "Puzzle 2: The canonical dangerous ingredient list: {}",
        result2
    );
}

pub fn puzzle2() {
    // Part of puzzle 1, because we need the same map anyway
}
