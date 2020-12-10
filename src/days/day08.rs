use crate::util::input::read_raw_input;

struct Instruction {
    op: String,
    val: i32,
}

fn parse_line(line: String) -> Result<Instruction, String> {
    let parts: Vec<_> = line.split(" ").map(str::to_owned).collect();
    if parts.len() != 2 {
        return Err(format!(
            "Expected line with op and value... but got {} parts in '{}'",
            parts.len(),
            line
        ));
    }

    let op = parts[0].to_string();
    let val = parts[1].parse::<i32>().map_err(|e| format!("{}", e))?;

    Ok(Instruction { op, val })
}

fn parse_program(program: String) -> Result<Vec<Instruction>, String> {
    let lines: Result<Vec<_>, _> = program
        .split("\n")
        .map(str::to_owned)
        .map(parse_line)
        .collect();
    lines
}

fn find_loop(program: &Vec<Instruction>) -> i32 {
    let mut acc = 0;
    let mut idx = 0;

    let mut seen: Vec<i32> = vec![];

    // We run the program until we encounter a loop. The result is the accumulator value at that point.
    loop {
        if seen.contains(&idx) {
            println!("Found loop, executing {} again!", idx);
            break;
        }

        seen.push(idx);
        let instr = &program[idx as usize];
        match &instr.op[0..3] {
            "acc" => {
                acc += instr.val;
                idx += 1;
                continue;
            }
            "jmp" => {
                idx += instr.val;
                continue;
            }
            _ => {
                idx += 1;
                continue;
            } // Consider anything unknown as 'NOP'
        }
    }

    acc
}

pub fn puzzle1() {
    match read_raw_input(8)
        .and_then(parse_program)
        .map(|p| find_loop(&p))
    {
        Err(e) => eprintln!("{}", e),
        Ok(result) => println!("Puzzle 1 result = {}", result),
    }
}

fn try_run_program(program: Vec<&Instruction>) -> Result<i32, ()> {
    let mut acc = 0;
    let mut idx = 0;

    let mut seen: Vec<i32> = vec![];

    // We run the program until we encounter a loop. The result is the accumulator value at that point.
    loop {
        if seen.contains(&idx) {
            return Err(());
        }

        if (idx as usize) >= program.len() {
            // Successful execution
            return Ok(acc);
        }

        seen.push(idx);
        let instr = &program[idx as usize];
        match &instr.op[0..3] {
            "acc" => {
                acc += instr.val;
                idx += 1;
                continue;
            }
            "jmp" => {
                idx += instr.val;
                continue;
            }
            _ => {
                idx += 1;
                continue;
            } // Consider anything unknown as 'NOP'
        }
    }
}

fn try_fix_program(program: Vec<Instruction>) -> Result<i32, String> {
    // We should be able to fix this program by changing either a 'jmp' to 'nop' or a 'nop' to 'jmp'
    // With just one of such a change, we should be able to run the program to end successfully.
    // Given the simpleness of this, we'll just try changing them one by one, and seeing if the program
    // runs successfully or loops.

    // Since we know we need to change one (puzzle 1 shows it loops), we'll replace them one by one
    // until we have a success:
    for i in 0..program.len() {
        if program[i].op.eq("acc") {
            // Only a JMP or NOP is wrong, so skip the ACC statements.
            continue;
        }

        let new_instruction = if program[i].op.eq("jmp") {
            "nop".to_owned()
        } else {
            "jmp".to_owned()
        };

        let new_instruction = vec![Instruction {
            op: new_instruction,
            val: program[i].val,
        }];

        let new_program: Vec<_> = program[0..i]
            .iter()
            .chain(new_instruction.iter())
            .chain(program[i + 1..program.len()].iter())
            .collect();

        match try_run_program(new_program) {
            Ok(v) => return Ok(v),
            Err(_) => continue,
        }
    }

    Err(format!(
        "Could not find a running program by changing one op..."
    ))
}

pub fn puzzle2() {
    match read_raw_input(8)
        .and_then(parse_program)
        .and_then(try_fix_program)
    {
        Err(e) => eprintln!("{}", e),
        Ok(result) => println!("Puzzle 2 result = {}", result),
    }
}
