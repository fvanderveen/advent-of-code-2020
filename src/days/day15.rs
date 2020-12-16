use std::collections::HashMap;

fn get_nth_number(n: i32) -> i32 {
    // A game of memory. This one seems even more horrible than Jip-Klaas-Janneke :joy:
    let mut memory = HashMap::new();

    // Pre-fill the memory with the seed values:
    memory.insert(2, 1);
    memory.insert(1, 2);
    memory.insert(10, 3);
    memory.insert(11, 4);
    memory.insert(0, 5);

    let mut round = 7;
    let mut last_number = 6;

    while round <= n {
        // So, rules.
        // If the last_number has never been spoken before, announce '0'
        // Otherwise, get the last round the number was spoken and announce the age (round - value)
        // e.g. we start with 6, which was never spoken before (we didn't insert this seed in the
        // memory for good reason). So we'll announce '0' (set last_number to 0) and insert 6 for
        // the previous round.
        // Then, the second time around (round 8), last_number is 0, which is in memory with round 5
        // so we'll announce 7 (when it was spoken) - 5 = 2

        match memory.insert(last_number, round - 1) {
            Some(last_round) => {
                last_number = round - 1 - last_round;
            }
            None => {
                // It was new!
                last_number = 0;
            }
        }

        round += 1;
    }

    last_number
}

pub fn puzzle1() {
    println!(
        "Puzzle 1: The 2020th number announced is: {}",
        get_nth_number(2020)
    );
}

pub fn puzzle2() {
    println!(
        "Puzzle 2: The 30.000.000th number announced is: {}",
        get_nth_number(30_000_000)
    );
}
