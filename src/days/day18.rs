use crate::days::day18::Operator::{ADDITION, MULTIPLY};
use crate::days::day18::Token::{Lit, Op, ParClose, ParOpen};
use crate::util::input::read_raw_input;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Operator {
    ADDITION,
    MULTIPLY,
}

fn eval(expression: &str) -> Result<u128, String> {
    // The expression is a string containing numbers, operators, and parenthesis.
    // Contrary to normal, the evaluation order is parenthesis first, the left-to-right
    // e.g. 1 + 2 * 3 + 4 => 3 * 3 + 4 => 9 + 4 => 13
    // and  1 + (2 * 3) + 4 => 1 + 6 + 4 => 7 + 4 => 11

    let mut result = 0;

    let mut operator = ADDITION;
    let mut index = 0;

    while index < expression.len() {
        match expression.chars().nth(index) {
            Some('*') => {
                operator = MULTIPLY;
                index += 1
            }
            Some('+') => {
                operator = ADDITION;
                index += 1
            }
            Some(v) if v.is_digit(10) => {
                // (Start of) a number, find all digit numbers and handle the operator
                let num_str = expression
                    .chars()
                    .skip(index)
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();
                let val = num_str
                    .parse::<u128>()
                    .map_err(|pe| format!("Could not parse {} as number: {}", num_str, pe))?;

                match operator {
                    ADDITION => result += val,
                    MULTIPLY => result *= val,
                }

                index += num_str.len();
            }
            Some('(') => {
                // Find the matching(!) closing brace, and evaluate the substring
                let mut brace_level = 0;
                let sub_expression = expression
                    .chars()
                    .skip(index + 1)
                    .take_while(|c| match c {
                        '(' => {
                            brace_level += 1;
                            true
                        }
                        ')' => {
                            brace_level -= 1;
                            brace_level == 0
                        }
                        _ => true,
                    })
                    .collect::<String>();
                let val = eval(sub_expression.as_str())?;
                match operator {
                    ADDITION => result += val,
                    MULTIPLY => result *= val,
                }
                index += sub_expression.len() + 2;
            }
            Some(v) if v.is_whitespace() => index += 1, // Skip whitespace
            Some(v) => {
                return Err(format!(
                    "Erroneous input, unexpected char '{}' at index {}",
                    v, index
                ));
            }
            None => return Err(format!("Expected a character at index {}?!", index)),
        }
    }

    Ok(result)
}

#[test]
fn test_eval() {
    assert_eq!(eval("1 + 2 + 3 + 4"), Ok(10));
    assert_eq!(eval("1 + 2 * 3 + 4"), Ok(13));
    assert_eq!(eval("1 + (2 * 3) + 4"), Ok(11));
    assert_eq!(eval("42 * (1 + (3 * 3)) + 4 + 5"), Ok(429))
}

pub fn puzzle1() {
    match read_raw_input(18)
        .and_then(|d| d.split("\n").map(eval).collect::<Result<Vec<_>, _>>())
        .map(|answers| answers.iter().sum::<u128>())
    {
        Err(e) => panic!("{}", e),
        Ok(v) => println!("Puzzle 1: Sum of all answers: {}", v),
    };
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Token {
    Lit(u128),
    ParOpen,
    ParClose,
    Op(Operator),
}

fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut index = 0;
    while index < expression.len() {
        match expression.chars().nth(index) {
            Some('(') => {
                tokens.push(ParOpen);
                index += 1
            }
            Some(')') => {
                tokens.push(ParClose);
                index += 1
            }
            Some('*') => {
                tokens.push(Op(MULTIPLY));
                index += 1
            }
            Some('+') => {
                tokens.push(Op(ADDITION));
                index += 1
            }
            Some(v) if v.is_digit(10) => {
                let num_str = expression
                    .chars()
                    .skip(index)
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();
                let num = num_str
                    .parse::<u128>()
                    .map_err(|p| format!("Could not parse {} to a number: {}", num_str, p))?;
                tokens.push(Lit(num));
                index += num_str.len();
            }
            Some(v) if v.is_whitespace() => {
                index += 1; /* ignore whitespace */
            }
            Some(v) => return Err(format!("Unexpected character '{}' at index {}", v, index)),
            None => return Err(format!("Expected a character at index {}", index)),
        }
    }

    Ok(tokens)
}

fn eval_tokens(tokens: &[Token]) -> u128 {
    // We'll first want to evaluate and replace the braced parts.
    fn expand_braces(tokens: &[Token]) -> Vec<Token> {
        let mut result = vec![];
        let mut par_start = 0;
        let mut par_level = 0;

        for i in 0..tokens.len() {
            match tokens[i] {
                ParOpen => {
                    par_level += 1;
                    if par_level == 1 {
                        par_start = i + 1;
                    }
                }
                ParClose => {
                    par_level -= 1;
                    if par_level == 0 {
                        result.push(Lit(eval_tokens(&tokens[par_start..i])));
                    }
                }
                v if par_level == 0 => {
                    // Any token not in parenthesis is just kept:
                    result.push(v);
                }
                _ => { /* inside parenthesis, will be evaluated on ParClose */ }
            }
        }

        result
    }

    // Then, without braces, we simply start looking for and replacing the additions!
    fn compute_addition(tokens: &[Token]) -> Vec<Token> {
        let mut result = vec![];

        let mut previous = tokens[0];

        let mut index = 1;
        while index < tokens.len() {
            match tokens[index] {
                Op(ADDITION) => {
                    // HANDLE!
                    let left = match previous {
                        Lit(v) => v,
                        t => panic!("Expected a literal LHS, but got {:?}", t),
                    };
                    let right = match tokens[index + 1] {
                        Lit(v) => v,
                        t => panic!("Expected a literal RHS, but got {:?}", t),
                    };
                    previous = Lit(left + right);
                    index += 2;
                }
                t @ Op(_) => {
                    // Push the previous and this token on the result, set previous to the next token
                    result.push(previous);
                    result.push(t);
                    previous = tokens[index + 1]; // Just panic if it's too far; should not happen.
                    index += 2;
                }
                t => panic!("Expected to find only operations, but encountered {:?}", t),
            }
        }

        result.push(previous);

        result
    }

    // Finally, find and replace the multiplications.
    fn compute_multiplication(tokens: &[Token]) -> Vec<Token> {
        let mut result = vec![];

        let mut previous = tokens[0];

        let mut index = 1;
        while index < tokens.len() {
            match tokens[index] {
                Op(MULTIPLY) => {
                    // HANDLE!
                    let left = match previous {
                        Lit(v) => v,
                        t => panic!("Expected a literal LHS, but got {:?}", t),
                    };
                    let right = match tokens[index + 1] {
                        Lit(v) => v,
                        t => panic!("Expected a literal RHS, but got {:?}", t),
                    };
                    previous = Lit(left * right);
                    index += 2;
                }
                t => panic!(
                    "Expected to find only MULTIPLY operands, but encountered {:?}",
                    t
                ),
            }
        }

        result.push(previous);

        result
    }

    let result = compute_multiplication(&compute_addition(&expand_braces(tokens)));
    if result.len() != 1 {
        panic!("Expected a single literal result, but got: {:?}", result)
    }

    match result[0] {
        Lit(v) => v,
        t => panic!("Expected a single literal result, but got: {:?}", t),
    }
}

fn eval2(expression: &str) -> Result<u128, String> {
    // Similar as before, but now the order is: parenthesis => addition => multiplication
    // e.g. 2 * 3 + 3 * 6 => 2 * 6 * 6 => 12 * 6 => 72
    // and  (2 * 3) + (3 * 6) => 6 + 18 => 24

    // Tokenize the expression:
    let tokens = tokenize(expression)?;
    // Evaluate:
    Ok(eval_tokens(&tokens))
}

#[test]
fn test_eval2() {
    assert_eq!(eval2("2 * 3 + 3 * 6"), Ok(72));
    assert_eq!(eval2("(2 * 3) + 3 * 6"), Ok(54));
    assert_eq!(eval2("(2 * 3) + (3 * 6)"), Ok(24));
}

pub fn puzzle2() {
    match read_raw_input(18)
        .and_then(|d| d.split("\n").map(eval2).collect::<Result<Vec<_>, _>>())
        .map(|answers| answers.iter().sum::<u128>())
    {
        Err(e) => panic!("{}", e),
        Ok(v) => println!("Puzzle 2: Sum of all answers: {}", v),
    };
}
