use crate::util::input::read_raw_input;

fn read_input(data: String) -> Result<(u128, Vec<u128>), String> {
    let lines = data.split("\n").collect::<Vec<_>>();
    if lines.len() > 2 {
        return Err(format!("Expected 2 lines, but read {}", lines.len()));
    }

    let arrival_time = lines[0]
        .parse::<u128>()
        .map_err(|e| format!("Could not parse arrival time: {}", e))?;
    let bus_lines = lines[1]
        .split(",")
        .map(|e| e.parse::<u128>())
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    Ok((arrival_time, bus_lines))
}

pub fn puzzle1() {
    let (arrival_time, bus_lines) = match read_raw_input(13).and_then(read_input) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // Each bus line is a number that indicates the length of its route (e.g. 5 departs at 0, 5, 10, etc)
    // We need to find the bus line closest to our `arrival_time`
    let result = bus_lines
        .iter()
        .map(|l| (l, l - (arrival_time % l)))
        .min_by(|(_, a_arrives_in), (_, b_arrives_in)| a_arrives_in.cmp(b_arrives_in));

    match result {
        None => eprintln!("Puzzle 1: Could not find any bus line?!"),
        Some((bus_line, arrives_in)) => println!(
            "Puzzle 2: line {} arrives in {}. Result = {}",
            bus_line,
            arrives_in,
            bus_line * arrives_in
        ),
    }
}

pub fn puzzle2() {
    let raw = match read_raw_input(13) {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    let lines = match raw.split("\n").nth(1).map(|l| {
        l.split(",")
            .map(|v| v.parse::<u128>().ok())
            .collect::<Vec<_>>()
    }) {
        Some(l) => l,
        None => {
            eprintln!("Could not extract bus lines...");
            return;
        }
    };

    // We need to find a timestamp T where:
    // lines[0] arrives at T
    // lines[1] arrives at T+1
    // ...
    // lines[n] arrives at T+n
    // Lines that are 'x' (Option.None) can be ignored.

    // We'll first find a point where the first two buses match up correctly.
    // From there on, we'll take increments of their LCM, as this is a cycle that will simply repeat.
    // We do this until the third bus aligns, and with the LCM of all three, we continue.
    // Do this until we have all buses aligned = win

    // First, let's get a list with each line and offset:
    let mut lines_with_offset: Vec<(u128, u128)> = vec![];
    for i in 0..lines.len() {
        if let Some(line) = lines[i] {
            lines_with_offset.push((line, i as u128))
        }
    }

    // Now lets find these alignments!
    let mut index = 1;
    let (mut cycle, offset) = lines_with_offset[0];
    let mut t = offset;

    loop {
        t += cycle;

        let (next_line, next_offset) = lines_with_offset[index];

        // Check if the next line is aligned:
        if ((t + next_offset) % next_line) != 0 {
            continue;
        }

        // If we're aligned and the last bus; we're done!
        if index == lines_with_offset.len() - 1 {
            println!("Aligned the last bus!");
            break;
        }

        println!("Aligned bus #{}", index + 1);

        // If so, find a new LCM between the current cycle and the next_line; and use that cycle
        cycle = lcm(cycle, next_line);
        index += 1;
    }

    println!("Puzzle 2: result = {}", t);
}

fn lcm(left: u128, right: u128) -> u128 {
    let numerator = left * right;
    let denominator = gcd(left, right);

    numerator / denominator
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}
