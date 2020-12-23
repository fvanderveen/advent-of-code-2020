use std::collections::{HashMap, LinkedList};

pub fn puzzle1() {
    let mut cups: LinkedList<i32> = LinkedList::new();
    cups.push_back(1);
    cups.push_back(5);
    cups.push_back(8);
    cups.push_back(9);
    cups.push_back(3);
    cups.push_back(7);
    cups.push_back(4);
    cups.push_back(6);
    cups.push_back(2);

    // Play game with the cups, we start at index 0 as current cup (and will move "clockwise"
    // that is; index + 1, with wraparound).
    // Each round has the following actions:
    // 1. Take the three cups clockwise next to current (with wraparound)
    // 2. Subtract one from the current cup number until a cup is found that is still in the circle (with wraparound)
    // 3. Place the three cups (in same order) directly clockwise next to the cup found in 2
    // 4. Move current one position clockwise.

    for _round in 0..100 {
        let current = cups.pop_front().unwrap();
        let mut next_cups = cups.split_off(3); // cups will be just what we picked up now

        let mut target = if current == 1 { 9 } else { current - 1 };
        while cups.contains(&target) {
            target = if target == 1 { 9 } else { target - 1 };
        }

        let index = next_cups.iter().position(|&v| v == target).unwrap();

        let mut remainder = next_cups.split_off(index + 1);

        next_cups.append(&mut cups);
        next_cups.append(&mut remainder);
        next_cups.push_back(current);

        cups = next_cups;
    }

    // Now move cup 1 to the front:
    while cups.front().unwrap().ne(&1) {
        let removed = cups.pop_front().unwrap();
        cups.push_back(removed);
    }

    let result = cups.iter().skip(1).map(i32::to_string).collect::<String>();
    println!(
        "Puzzle 1: Order of cups after 1 after 100 rounds: {}",
        result
    );
}

// Linked list is not fast enough.
#[derive(Eq, PartialEq)]
struct Item<'a> {
    value: i32,
    next: Option<&'a mut Item<'a>>,
}

pub fn puzzle2() {
    let input = vec![/*1, */ 5, 8, 9, 3, 7, 4, 6, 2];

    let mut cups = HashMap::new();
    let mut previous = 1;
    for item in input {
        cups.insert(previous, item);
        previous = item;
    }
    for item in 10..=1_000_000 {
        cups.insert(previous, item);
        previous = item;
    }
    cups.insert(previous, 1);

    // Okay, game plan again!
    let mut current = 1;

    // Make it cycle!

    for i in 0..10_000_000 {
        if i % 100_000 == 99_999 {
            println!("Round {}", i + 1);
        }

        fn get_values(current: i32, map: &HashMap<i32, i32>) -> (i32, i32, i32, i32) {
            let picked1 = map.get(&current).unwrap().clone();
            let picked2 = map.get(&picked1).unwrap().clone();
            let picked3 = map.get(&picked2).unwrap().clone();
            let next = map.get(&picked3).unwrap().clone();
            return (picked1, picked2, picked3, next);
        }

        let (picked1, picked2, picked3, next) = get_values(current, &cups);

        fn get_insert(
            current: i32,
            p1: i32,
            p2: i32,
            p3: i32,
            map: &HashMap<i32, i32>,
        ) -> (i32, i32) {
            let mut insert = if current == 1 { 1_000_000 } else { current - 1 };
            while insert == p1 || insert == p2 || insert == p3 {
                insert = if insert == 1 { 1_000_000 } else { insert - 1 };
            }

            let next = map.get(&insert).unwrap().clone();
            (insert, next)
        }

        let (insert, insert_tail) = get_insert(current, picked1, picked2, picked3, &cups);

        cups.insert(current, next);
        cups.insert(insert, picked1);
        cups.insert(picked3, insert_tail);

        current = next;
    }

    // Find the two cups next to 1:
    let first = cups.get(&1).unwrap();
    let second = cups.get(first).unwrap();

    println!(
        "Puzzle 2: The cups {} and {} are next to 1, result = {}",
        first,
        second,
        first.clone() as usize * second.clone() as usize
    );
}
