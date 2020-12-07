use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input_file() -> Result<String, String> {
    let data = read_to_string("input/day7.txt");
    return match data {
        Err(err) => Err(err.to_string()),
        Ok(data) => {
            return Ok(data);
        }
    };
}

#[derive(Eq, PartialEq, Debug)]
struct Content {
    bag_type: String,
    amount: i32,
}

#[derive(PartialEq, Debug)]
enum ParseState {
    Init,
    BagType,
    BagsToken,
    ContainToken,
    NoToken,
    OtherToken,
    ContentBagType,
    ContentComma,
    EOL,
}

fn parse_line(line: &str) -> Result<(String, Vec<Content>), String> {
    let words: Vec<_> = line.split(" ").collect();

    let mut state = ParseState::Init;
    let mut bag_type: String = "".to_owned();
    let mut content_amount = 0;
    let mut content_bag_type: String = "".to_owned();
    let mut contents: Vec<Content> = vec![];

    for word in words {
        match word {
            // Match token values, and validate/move state:
            "bags" => {
                if state != ParseState::BagType {
                    return Err(format!(
                        "Expected to read a bag type before finding 'bags', but got {:?}",
                        state
                    ));
                }

                state = ParseState::BagsToken;
            }
            "contain" => {
                if state != ParseState::BagsToken {
                    return Err(format!(
                        "Expected 'bags' before reading 'contain', but got {:?}",
                        state
                    ));
                }
                state = ParseState::ContainToken;
            }
            "no" => {
                if state != ParseState::ContainToken {
                    return Err(format!(
                        "Expected 'contain' before reading 'no', but got {:?}",
                        state
                    ));
                }
                state = ParseState::NoToken;
            }
            "other" => {
                if state != ParseState::NoToken {
                    return Err(format!(
                        "Expected 'no' before reading 'other', but got {:?}",
                        state
                    ));
                }
                state = ParseState::OtherToken;
            }
            "bags." if state == ParseState::OtherToken => {
                state = ParseState::EOL;
            }
            "bags," | "bag," => {
                if state != ParseState::ContentBagType {
                    return Err(format!(
                        "Expected a bag type before reading '{}', but got {:?}",
                        word, state
                    ));
                }

                contents.push(Content {
                    bag_type: content_bag_type,
                    amount: content_amount,
                });
                content_bag_type = "".to_owned();
                state = ParseState::ContentComma;
            }
            "bags." | "bag." => {
                if state != ParseState::ContentBagType {
                    return Err(format!(
                        "Expected a bag type before reading '{}', but got {:?}",
                        word, state
                    ));
                }

                contents.push(Content {
                    bag_type: content_bag_type,
                    amount: content_amount,
                });
                content_bag_type = "".to_owned();
                state = ParseState::EOL;
            }
            // Match other data, and fill what we have:
            _ => {
                match state {
                    ParseState::EOL => return Err(format!("Found extra input after EOL")),
                    ParseState::Init | ParseState::BagType => {
                        bag_type = format!("{} {}", bag_type, word);
                        bag_type = bag_type.trim().to_owned();
                        state = ParseState::BagType;
                    }
                    ParseState::ContainToken | ParseState::ContentComma => {
                        // We expect this word to be numeric.
                        content_amount = match word.parse::<i32>() {
                            Err(e) => return Err(format!("{}", e)),
                            Ok(v) => v,
                        };
                        state = ParseState::ContentBagType;
                    }
                    ParseState::ContentBagType => {
                        content_bag_type = format!("{} {}", content_bag_type, word);
                        content_bag_type = content_bag_type.trim().to_owned();
                        state = ParseState::ContentBagType;
                    }
                    _ => return Err(format!("Found invalid input after {:?}", state)),
                }
            }
        }
    }

    Ok((bag_type, contents))
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("shiny blue bags contain no other bags."),
        Ok(("shiny blue".to_owned(), vec![]))
    );
    assert_eq!(
        parse_line("shiny blue bags contain 2 mat green bags, 1 silver bag."),
        Ok((
            "shiny blue".to_owned(),
            vec![
                Content {
                    bag_type: "mat green".to_owned(),
                    amount: 2
                },
                Content {
                    bag_type: "silver".to_owned(),
                    amount: 1
                }
            ]
        ))
    );
    assert_eq!(
        parse_line("shiny blue bags contain 42 mat green bags."),
        Ok((
            "shiny blue".to_owned(),
            vec![Content {
                bag_type: "mat green".to_owned(),
                amount: 42
            }]
        ))
    );
}

fn read_input_data() -> Result<HashMap<String, Vec<Content>>, String> {
    let data = read_input_file()?;
    // Each line contains either:
    // - <bag_type> bags contain <# bag_type>[, …].
    // - <bag_type> bags contain no other bags.

    // We'll parse the input line-by-line, word-by-word, trying to make sense of it.
    let lines: Vec<_> = data.split("\n").collect();

    let mut result: HashMap<String, Vec<Content>> = HashMap::with_capacity(lines.len());

    for line in lines {
        let (bag_type, contents) = match parse_line(line) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        // Create entry using bag_type:
        match result.insert(bag_type.to_owned(), contents) {
            Some(_) => return Err(format!("Already read an entry for {}", bag_type)),
            None => {}
        }
    }

    Ok(result)
}

pub fn puzzle1() {
    let bags = match read_input_data() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // We need to find the number of (unique) bag types that can contain a shiny gold bag.
    // We'll need to find all bags where the content defined a 'shiny gold' bag.
    // Then, we'll iterate again on bags containing those bags (ignored the ones we've already seen)
    let mut seen: Vec<String> = vec![];
    let mut search: Vec<String> = vec!["shiny gold".to_owned()];

    loop {
        let mut new_search: Vec<String> = vec![];
        for (key, value) in &bags {
            if seen.contains(&key) {
                continue; // Already searched before
            }

            if !value.iter().any(|c| search.contains(&c.bag_type)) {
                continue; // No contents of interest currently.
            }

            seen.push(key.to_string());
            new_search.push(key.to_string());
        }

        if new_search.len() == 0 {
            break;
        }
        search = new_search;
    }

    println!(
        "Puzzle 1: I've visited {} bags that should be able to contain a shiny gold bag",
        seen.len()
    );
}

fn get_bag_count(map: &HashMap<String, Vec<Content>>, bag: &String) -> Result<i32, String> {
    let contents = match map.get(bag) {
        None => {
            return Err(format!("Needed to find {}, but not found in input", bag));
        }
        Some(c) => c,
    };

    let mut amount = 0;
    for content in contents {
        amount += content.amount + (content.amount * get_bag_count(map, &content.bag_type)?);
    }

    Ok(amount)
}

#[test]
fn test_get_bag_count() {
    let mut bags: HashMap<String, Vec<Content>> = HashMap::new();
    bags.insert("red".to_owned(), vec![]);
    bags.insert(
        "blue".to_owned(),
        vec![Content {
            bag_type: "red".to_owned(),
            amount: 3,
        }],
    );
    bags.insert(
        "green".to_owned(),
        vec![
            Content {
                bag_type: "red".to_owned(),
                amount: 2,
            },
            Content {
                bag_type: "blue".to_owned(),
                amount: 1,
            },
        ],
    );

    assert_eq!(get_bag_count(&bags, &"red".to_owned()), Ok(0));
    assert_eq!(get_bag_count(&bags, &"blue".to_owned()), Ok(3));
    assert_eq!(get_bag_count(&bags, &"green".to_owned()), Ok(6));
}

pub fn puzzle2() {
    let bags = match read_input_data() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // We need to figure out the amount of bags we need _inside_ our shiny gold bag. (Oh dear)
    let result = match get_bag_count(&bags, &"shiny gold".to_owned()) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    println!(
        "Puzzle 2: To fill a shiny gold bag, we need {} other bags",
        result
    );
}
