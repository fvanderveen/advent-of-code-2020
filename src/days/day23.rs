pub fn puzzle1() {
    let input = vec![/*1, */ 5, 8, 9, 3, 7, 4, 6, 2];

    let mut cups = [0; 10];
    let mut previous = 1;
    for item in input {
        cups[previous] = item;
        previous = item;
    }
    cups[previous] = 1;

    // Play game with the cups, we start at index 0 as current cup (and will move "clockwise"
    // that is; index + 1, with wraparound).
    // Each round has the following actions:
    // 1. Take the three cups clockwise next to current (with wraparound)
    // 2. Subtract one from the current cup number until a cup is found that is still in the circle (with wraparound)
    // 3. Place the three cups (in same order) directly clockwise next to the cup found in 2
    // 4. Move current one position clockwise.

    let mut current = 1;
    for _i in 0..100 {
        let picked1 = cups[current];
        let picked2 = cups[picked1];
        let picked3 = cups[picked2];
        let next = cups[picked3];

        let mut insert = if current == 1 { 9 } else { current - 1 };
        while insert == picked1 || insert == picked2 || insert == picked3 {
            insert = if insert == 1 { 9 } else { insert - 1 };
        }
        let insert_tail = cups[insert];

        cups[current] = next;
        cups[insert] = picked1;
        cups[picked3] = insert_tail;

        current = next;
    }

    let mut result = vec![];
    let mut current = cups[1];
    while current != 1 {
        result.push(current);
        current = cups[current];
    }

    println!(
        "Puzzle 1: Order of cups after 1 after 100 rounds: {}",
        result.iter().map(usize::to_string).collect::<String>()
    );
}

pub fn puzzle2() {
    let input = vec![/*1, */ 5, 8, 9, 3, 7, 4, 6, 2];

    let mut cups = [0; 1_000_001];
    let mut previous = 1;
    for item in input {
        cups[previous] = item;
        previous = item;
    }
    for item in 10..=1_000_000 {
        cups[previous] = item;
        previous = item;
    }

    cups[previous] = 1;

    // Okay, game plan again!
    let mut current = 1;

    // Make it cycle!

    for _i in 0..10_000_000 {
        let picked1 = cups[current];
        let picked2 = cups[picked1];
        let picked3 = cups[picked2];
        let next = cups[picked3];

        let mut insert = if current == 1 { 1_000_000 } else { current - 1 };
        while insert == picked1 || insert == picked2 || insert == picked3 {
            insert = if insert == 1 { 1_000_000 } else { insert - 1 };
        }
        let insert_tail = cups[insert];

        cups[current] = next;
        cups[insert] = picked1;
        cups[picked3] = insert_tail;

        current = next;
    }

    // Find the two cups next to 1:
    let first = cups[1];
    let second = cups[first];

    println!(
        "Puzzle 2: The cups {} and {} are next to 1, result = {}",
        first,
        second,
        first * second
    );
}
