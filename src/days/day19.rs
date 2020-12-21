use crate::days::day19::Rule::{Lit, Or, Ref, Seq};
use crate::util::input::read_raw_input;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug)]
enum Rule {
    Or(Vec<Rule>),
    Seq(Vec<Rule>),
    Ref(u32),
    Lit(char),
}

impl Clone for Rule {
    fn clone(&self) -> Rule {
        match self {
            Or(rules) => Or(rules.iter().map(Rule::clone).collect()),
            Seq(sequence) => Seq(sequence.iter().map(Rule::clone).collect()),
            Ref(rule_id) => Ref(rule_id.clone()),
            Lit(c) => Lit(c.clone()),
        }
    }
}

fn parse_sequence(sequence: &str) -> Result<Rule, String> {
    let rules = sequence
        .split(" ")
        .map(str::trim)
        .map(|r| match r.chars().nth(0) {
            Some(v) if v.is_digit(10) => {
                // Parse as ref
                Ok(Ref(r.parse::<u32>().map_err(|e| {
                    format!("Could not parse '{}' as a rule ID: {}", r, e)
                })?))
            }
            Some('"') if r.len() == 3 => {
                // Parse as lit
                Ok(Lit(r.chars().nth(1).ok_or(format!(
                    "Could not extract literal from '{}'",
                    r
                ))?))
            }
            _ => Err(format!("Could not parse '{}' as a rule", r)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Seq(rules))
}

fn parse_rule(rule: &str) -> Result<(u32, Rule), String> {
    // A rule is formatted as follows:
    // Starts with the rule ID followed by a colon(:)
    // Then, one (or more, when separated by '|') sequence to match.
    // A sequence has either references to other rules (numbers), or a literal to match ("a")

    // First, get the ID from the rule:
    let rule_id = rule
        .chars()
        .take_while(|c| c.ne(&':'))
        .collect::<String>()
        .parse::<u32>()
        .map_err(|e| format!("Could not parse rule ID: {} (parsing '{}')", e, rule))?;

    // Secondly, grab the string after the ':'
    let sequences = rule
        .chars()
        .skip_while(|c| c.ne(&':'))
        .skip(1)
        .collect::<String>()
        // and split that on '|'
        .split("|")
        .map(|p| p.trim())
        // and parse all parts to a sequence:
        .map(parse_sequence)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((rule_id, Or(sequences)))
}

fn parse_rules(rules: &String) -> Result<HashMap<u32, Rule>, String> {
    let rules = rules
        .split("\n")
        .map(parse_rule)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HashMap::from_iter(rules))
}

fn matches(input: &str, rules: &HashMap<u32, Rule>) -> bool {
    // Return true if we could find a path trough the rules that fully matches input.
    fn try_match(input: &str, index: usize, todo: &[Rule], rules: &HashMap<u32, Rule>) -> bool {
        // Attempt matching the input from the given index to the given rule.
        if todo.len() == 0 {
            // At the end of the work, check if we matched all characters.
            return index == input.len();
        }

        let rule = &todo[0];
        let next = &todo[1..];

        match rule {
            Lit(lit) => {
                return if let Some(c) = input.chars().nth(index) {
                    c.eq(lit) && try_match(input, index + 1, next, rules)
                } else {
                    false
                }
            }
            Ref(rule_id) => rules
                .get(&rule_id)
                .map(|r| {
                    try_match(
                        input,
                        index,
                        &vec![r]
                            .into_iter()
                            .chain(next.into_iter())
                            .cloned()
                            .collect::<Vec<_>>(),
                        rules,
                    )
                })
                .unwrap_or(false),
            Seq(sequence) => {
                // This one will just flatten the list, by inserting the sequence in front of the queue
                return try_match(
                    input,
                    index,
                    &sequence
                        .iter()
                        .chain(next.iter())
                        .cloned()
                        .collect::<Vec<_>>(),
                    rules,
                );
            }
            Or(alternatives) => {
                // In puzzle 2 we get loops. This means that there are two rules where an arbitrary amount
                // of characters can be matched. To support that, we modify this function to know what the
                // next rules are; and for each alternative, we see if we can correctly match the rest of
                // the pattern.
                for alternative in alternatives {
                    // To find a working alternative, we replace the current rule by the alternative, and try matching that.
                    if try_match(
                        input,
                        index,
                        &vec![alternative]
                            .into_iter()
                            .chain(next.into_iter())
                            .cloned()
                            .collect::<Vec<_>>(),
                        rules,
                    ) {
                        return true;
                    }
                }
                return false;
            }
        }
    }

    rules
        .get(&0)
        .map(|r| try_match(input, 0, &vec![r.clone()], rules))
        .unwrap_or(false)
}

#[test]
fn test_matches() {
    // A test set of rules:
    let mut rules = HashMap::new();
    /*
    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"
     */
    rules.insert(0, Seq(vec![Ref(4), Ref(1), Ref(5)]));
    rules.insert(
        1,
        Or(vec![Seq(vec![Ref(2), Ref(3)]), Seq(vec![Ref(3), Ref(2)])]),
    );
    rules.insert(
        2,
        Or(vec![Seq(vec![Ref(4), Ref(4)]), Seq(vec![Ref(5), Ref(5)])]),
    );
    rules.insert(
        3,
        Or(vec![Seq(vec![Ref(4), Ref(5)]), Seq(vec![Ref(5), Ref(4)])]),
    );
    rules.insert(4, Lit('a'));
    rules.insert(5, Lit('b'));

    assert_eq!(matches("abbabb", &rules), true);
    assert_eq!(matches("aaabab", &rules), true);
    assert_eq!(matches("aaaabb", &rules), true);
    assert_eq!(matches("aabaab", &rules), true);
    assert_eq!(matches("babaab", &rules), false);
    assert_eq!(matches("aabaaa", &rules), false);
    assert_eq!(matches("aabaaba", &rules), false);
}

#[test]
fn test_matches_with_loop() {
    let mut rules = HashMap::new();
    /*
    0: 3 4
    1: "a"
    2: "b"
    3: 1 | 1 3
    4: 2 1 | 2 4 1
     */
    rules.insert(0, Seq(vec![Ref(3), Ref(4)]));
    rules.insert(1, Lit('a'));
    rules.insert(2, Lit('b'));
    rules.insert(3, Or(vec![Ref(1), Seq(vec![Ref(1), Ref(3)])]));
    rules.insert(
        4,
        Or(vec![
            Seq(vec![Ref(2), Ref(1)]),
            Seq(vec![Ref(2), Ref(4), Ref(1)]),
        ]),
    );

    // Minimal match:
    assert_eq!(matches("aba", &rules), true);
    assert_eq!(matches("aaaaaaaaba", &rules), true);
    assert_eq!(matches("abbbaaa", &rules), true);
    assert_eq!(matches("abbbaa", &rules), false);
    assert_eq!(matches("bbbaaa", &rules), false);
    assert_eq!(matches("aaaaabbbbbbaaaaaa", &rules), true);
}

#[test]
fn test_matches_with_loop_adv() {
    let mut rules = HashMap::new();
    /*
    0: 3 4
    1: "a"
    2: "b"
    3: 1 | 1 3
    4: 1 2 | 1 4 2
     */
    rules.insert(0, Seq(vec![Ref(3), Ref(4)]));
    rules.insert(1, Lit('a'));
    rules.insert(2, Lit('b'));
    rules.insert(3, Or(vec![Ref(1), Seq(vec![Ref(1), Ref(3)])]));
    rules.insert(
        4,
        Or(vec![
            Seq(vec![Ref(1), Ref(2)]),
            Seq(vec![Ref(1), Ref(4), Ref(2)]),
        ]),
    );

    // Minimal match:
    assert_eq!(matches("aab", &rules), true);
    // Repeats:
    assert_eq!(matches("aaaaaaaabb", &rules), true);
    assert_eq!(matches("aaaabbb", &rules), true);
    assert_eq!(matches("aaaaaaaaaaabbbbbb", &rules), true);
    // Failures:
    assert_eq!(matches("aaabbb", &rules), false);
    assert_eq!(matches("ab", &rules), false);
}

pub fn puzzle1() {
    let (rules, lines) = match read_raw_input(19).and_then(|d| {
        let parts = d.split("\n\n").map(str::to_owned).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Expected 2 parts of input, found {}", parts.len()));
        }
        let rules = parse_rules(&parts[0])?;
        let lines = parts[1].split("\n").map(str::to_owned).collect::<Vec<_>>();
        Ok((rules, lines))
    }) {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    let matches = lines
        .iter()
        .filter(|line| matches(line.as_str(), &rules))
        .count();
    println!(
        "Puzzle 1: There are {} lines matching the ruleset.",
        matches
    );
}

pub fn puzzle2() {
    let (mut rules, lines) = match read_raw_input(19).and_then(|d| {
        let parts = d.split("\n\n").map(str::to_owned).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Expected 2 parts of input, found {}", parts.len()));
        }
        let rules = parse_rules(&parts[0])?;
        let lines = parts[1].split("\n").map(str::to_owned).collect::<Vec<_>>();
        Ok((rules, lines))
    }) {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    // Adjust the rules as given by puzzle 2:
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rules.insert(8, Or(vec![Ref(42), Seq(vec![Ref(42), Ref(8)])]));
    rules.insert(
        11,
        Or(vec![
            Seq(vec![Ref(42), Ref(31)]),
            Seq(vec![Ref(42), Ref(11), Ref(31)]),
        ]),
    );

    let matches = lines
        .iter()
        .filter(|line| matches(line.as_str(), &rules))
        .count();
    println!(
        "Puzzle 2: There are {} lines matching the ruleset.",
        matches
    );
}
