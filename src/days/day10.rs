use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input_file() -> Result<String, String> {
    let data = read_to_string("input/day10.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            return Ok(data);
        }
    };
}

fn parse_input(input: String) -> Result<Vec<i32>, String> {
    input
        .split("\n")
        .map(|l| l.parse::<i32>().map_err(|e| format!("{}", e)))
        .collect()
}

pub fn puzzle1() {
    let adapters = match read_input_file().and_then(parse_input) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // Puzzle 1 wants us to use all adapters, and find the number of increases of 1 and 3.
    // The sockets starts at 0, and each adapter can jump 1, 2, or 3 up from the previous value.
    // Finally, after the highest adapter, there is a jump of 3 to the device.
    // The puzzle result is the amount of increases by 1 multiplied by those of 3.

    let mut sorted = adapters.clone();
    sorted.sort(); // we only have an in-place sorting function.

    let mut hops_of_one = 0;
    let mut hops_of_three = 1; // Init with one, as we know there is always a hop of three at the end

    for i in 0..sorted.len() {
        let previous = if i == 0 { 0 } else { sorted[i - 1] };
        let current = sorted[i];

        match current - previous {
            1 => hops_of_one += 1,
            3 => hops_of_three += 1,
            _ => {}
        }
    }

    println!(
        "Puzzle 1: {} hops of one, {} hops of three. Result = {}",
        hops_of_one,
        hops_of_three,
        hops_of_one * hops_of_three
    )
}

pub fn puzzle2() {
    let adapters = match read_input_file().and_then(parse_input) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // More fun. We need to find out in how many _different_ ways we can connect the socket (0) to
    // our device (highest value + 3).
    // There is a lot of inputs, so we need to be smart about this.

    let result = get_possible_connection_count(&adapters);

    println!(
        "Puzzle 2: Got {} possible ways to connect the adapters!",
        result
    );
}

// Now, the smart part...
// Assuming (0) -> 1 -> 2 -> 3 -> (6)
// - We know we can skip 1 (resulting in 1 additional option) (0) -> 2 -> 3 -> (6)
// - We know we can skip 1 & 2 (resulting in 1 more additional option) (0) -> 3 -> (6)
// - We know we can skip 2 (resulting in 1 additional option) (0) -> 1 -> 3 -> (6)
// - We can not skip 3, as 2->6 is impossible.
// So, here's what I hope works:
// Probably, we should start at the end. As mutations there are obviously possible for anything
// possible in front of them...
// Taking (0) -> 1 -> 2 -> 4 -> 6 -> 8 -> 9 -> 10 -> 11 -> 12 -> (15)
// We cannot skip 12, so much is clear (1 option)
// We can skip 11 (2 options)
// We can skip 10 (2 options), _but_ we can also skip 10 _and_ 11 (1 more option)
// We can skip 9 (2 options), we can skip 9 _and_ 10 (1 more), but not 9, 10, and 11. (< how can we catch this?)
// - We could look back for each option, if we can skip it, how many previous adapters we can also skip?
// However, what about skip 9, and 11? that's also valid! :cry:
// So, up to looking at 9, we have:
// 8 -> 9 -> 10 -> 11 -> 12 | initial
// 8 -> 9 -> 10 -> 12       | skip 11
// 8 -> 9 -> 11 -> 12       | skip 10
// 8 -> 9 -> 12             | skip 10 + 11
// 8 -> 10 -> 11 -> 12      | skip 9
// 8 -> 11 -> 12            | skip 9 + 10
// 8 -> 10 -> 12            | skip 9 + 11
// 6 -> 8 -> 9 …
// 6 -> 9 -> …
// Random thoughts:
// From 12, we can go to 9; there's two in between, allowing for 4 options to get from 9 to 12.
// Considering 9...
// - we can go to 6, with or without 8 (so that's 2 extra options on top of the 4 options = 8 options)
// - we can skip 9, but only if 10 or 11 is still included. (= 3/4 incoming cases)
// hmm
// 12 -> 1 option, cannot be skipped
// 11 -> +1 option, can be skipped
// 10 -> +1 option, can be skipped, +1 option, can skip 10+11, cannot skip 10+11+12
// 9 -> +1 option, can be skipped, +1 option, can skip 9+10, cannot skip 9+10+11
// 8 -> +1 option, can be skipped, cannot skip 8+9
// expected = 7
// Maybe we can do something with the difference?
// - a: Diff = 3 => nothing can change.
// - b: Diff = 2 => only if the next diff is 1
// - c: Diff = 1 => possible options:
//   - c1: next diff = 3 => cannot be skipped
//   - c2: next diff = 2 => can be skipped in one way
//   - c3: next diff = 1 => options!
//     - c3.a next diff = 1 => skippable in three options
//     - c3.b next diff = 2 => skippable in two option
//     - c3.c next diff = 3 => skippable in one option
// Do this for all options, and hope for the best!
// Example, looking at '1':
// a)     1 -> 4 = 1 option
// b)     1 -> 3 -> 4 = 2 options = skip next adapter once (in this case, just add the mutations from 1 and from 3?)
// b')    1 -> 3 -> 5 = 1 option
// c1)    1 -> 2 -> 5 = 1 options
// c2')   1 -> 2 -> 4 = 2 options (with, or without 2) = skip next adapter once
// c3.a)  1 -> 2 -> 3 -> 4 = 4 options (skip none, 2, 3, or both) = skip next adapter twice (2, 2+3)
// c3.b)  1 -> 2 -> 3 -> 5 = 3 options (skip none, 2, or 3) = skip next adapter once
// c3.c)  1 -> 2 -> 3 -> 6 = 2 options (skip none, or 2)  = skip next adapter once
// How to handle cases like C; because you want to check again on the next diff...
// Maybe only count how often 2 is skipped? However, if 2+3 is skipped, looking at 2->3 is weird.
// Then how to sum the options? Most likely multiplying?

// 0 -> 3 -> 6 -> 9
//    1    1    1   = 1
// Lets see here:
// 0 -> 2 -> 3 -> 5 -> 6
//   2    2    2    1   = 8?
// 0 -> 2 -> 3 = 2 options
// 2 -> 3 -> 5 = 2 options (only when 2 included) = 1 case so far
// 3 -> 5 -> 6 = 2 options (only when 3 included) = 2 cases
// 5 -> 6 = 1 option
// 1: 0 -> 2 -> 3 -> 5 -> 6
// 2: 0 -> 2 -> 3 -> 6
// 3: 0 -> 2 -> 5 -> 6
// 4: 0 -> 3 -> 5 -> 6
// 5: 0 -> 3 -> 6
// :(
// 6 = 1
// 5 = 1 * 1 (to 6)
// 3 = 1 * 1 to 6, and 1*1 from 5 = 2
// 2 = 1 * 2 to 3, 1 * 1 to 5 = 3
// 0 = 1 * 3 to 2, 1 * 2 to 3 = 5
// Hallelujah!

fn get_possible_connection_count(adapters: &Vec<i32>) -> i128 {
    // first, sort this thing...
    let mut sorted = adapters.clone();
    sorted.sort();
    sorted.reverse();

    // What we'll do; we create a mapping from the last number till the first, calculating
    // the amount of possibilities. The last one gets '1' (to the device adapter), the one before
    // that is also 1, as there is nothing to skip. From there on, we add all calculated values
    // together from the numbers we can reach.

    let mut value_map: HashMap<i32, i128> = HashMap::with_capacity(sorted.len());

    for i in 0..sorted.len() {
        let current = sorted[i];

        if i == 0 {
            // last adapter always has 1 option
            value_map.insert(current, 1);
            continue;
        }

        let mut options = 0;
        // Get previous adapters we can still connect to:
        for j in 1..=i {
            let next = sorted[i - j];
            if next - current > 3 {
                break;
            }

            if let Some(v) = value_map.get(&next) {
                options += v
            }
        }
        value_map.insert(current, options);
    }

    // Since 0 (the outlet) is not part of the adapters, we need to add the values of 1, 2, and 3
    // (if existing) together to get our result:
    let mut result = 0;

    if let Some(v) = value_map.get(&1) {
        result += v
    }
    if let Some(v) = value_map.get(&2) {
        result += v
    }
    if let Some(v) = value_map.get(&3) {
        result += v
    }

    result
}

#[test]
fn test_get_possible_connection_count() {
    assert_eq!(
        get_possible_connection_count(&vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
        8
    );
    assert_eq!(
        get_possible_connection_count(&vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3
        ]),
        19208
    );
}
