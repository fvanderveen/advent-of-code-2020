use crate::util::input::read_raw_input;

struct Group {
    answers: Vec<String>,
}

fn get_groups(input: &String) -> Vec<Group> {
    input
        .split("\n\n") // Groups are separated by a blank line
        .map(|d| Group {
            answers: d.split("\n").map(str::to_owned).collect(), // Each line contains the answers from one person
        })
        .collect()
}

fn get_group_answer_count(group: &Group) -> i32 {
    let mut result = vec![false; 26];

    for answer in &group.answers {
        for char in answer.chars() {
            let index = char.to_digit(36).unwrap() - 10;
            result[index as usize] = true;
        }
    }

    result.iter().map(|&v| if v { 1 } else { 0 }).sum()
}

#[test]
fn test_get_group_answer_count() {
    assert_eq!(
        get_group_answer_count(&Group {
            answers: vec!["abc".to_owned()]
        }),
        3
    );
    assert_eq!(
        get_group_answer_count(&Group {
            answers: vec!["abc".to_owned(), "bcd".to_owned()]
        }),
        4
    );
    assert_eq!(
        get_group_answer_count(&Group {
            answers: vec!["a".to_owned(), "a".to_owned(), "a".to_owned()]
        }),
        1
    );
    assert_eq!(
        get_group_answer_count(&Group {
            answers: vec!["a".to_owned(), "x".to_owned(), "z".to_owned()]
        }),
        3
    );
}

pub fn puzzle1() {
    // For each group, count the unique answers
    // Puzzle 1 output is the sum of those
    let groups = match read_raw_input(6) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => get_groups(&v),
    };

    let answer: i32 = groups.iter().map(get_group_answer_count).sum();
    println!("Puzzle 1 answer: {}", answer);
}

fn get_group_mutual_answer_count(group: &Group) -> usize {
    // Find the number of answers everyone in the group answered
    let mut results: Vec<usize> = vec![0; 26];

    for answer in &group.answers {
        for char in answer.chars() {
            let index = char.to_digit(36).unwrap() - 10;
            results[index as usize] += 1
        }
    }

    results
        .iter()
        .filter(|&n| n.eq(&group.answers.len()))
        .count()
}

#[test]
fn test_get_group_mutual_answer_count() {
    assert_eq!(
        get_group_mutual_answer_count(&Group {
            answers: vec!["abc".to_owned()]
        }),
        3
    );
    assert_eq!(
        get_group_mutual_answer_count(&Group {
            answers: vec!["abc".to_owned(), "bcd".to_owned()]
        }),
        2
    );
    assert_eq!(
        get_group_mutual_answer_count(&Group {
            answers: vec!["a".to_owned(), "a".to_owned(), "a".to_owned()]
        }),
        1
    );
    assert_eq!(
        get_group_mutual_answer_count(&Group {
            answers: vec!["a".to_owned(), "x".to_owned(), "z".to_owned()]
        }),
        0
    );
}

pub fn puzzle2() {
    // Oops. We needed the answers _everyone_ in the group answered!
    let groups = match read_raw_input(6) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => get_groups(&v),
    };

    let answer: usize = groups.iter().map(get_group_mutual_answer_count).sum();
    println!("Puzzle 2 answer: {}", answer);
}
