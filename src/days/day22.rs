use crate::util::input::read_raw_input;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::ops::AddAssign;

fn read_decks() -> Result<(Vec<usize>, Vec<usize>), String> {
    read_raw_input(22)
        .and_then(|data| {
            data.split("\n\n")
                .map(|d| {
                    // First line of the deck is just the 'player' line, so we ignore it!
                    d.split("\n")
                        .skip(1)
                        .map(str::trim)
                        .map(|c| c.parse::<usize>().map_err(|e| format!("{}", e)))
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<Vec<_>>, _>>()
        })
        .and_then(|v| {
            if v.len() == 2 {
                Ok((v[0].clone(), v[1].clone()))
            } else {
                Err(format!("Expected 2 players but got {}", v.len()))
            }
        })
}

pub fn puzzle1() {
    let (mut d1, mut d2) = match read_decks() {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    // 1. pull the top card of each deck (first index)
    // 2. put both cards (highest first) at the bottom of the winning player's deck
    // 3. play until one of the decks is empty

    while d1.len() > 0 && d2.len() > 0 {
        let p1 = d1[0];
        d1 = Vec::from(&d1[1..]);

        let p2 = d2[0];
        d2 = Vec::from(&d2[1..]);

        if p1 > p2 {
            // Player 1 wins!
            d1.push(p1);
            d1.push(p2);
        } else {
            // Player 2 wins!
            d2.push(p2);
            d2.push(p1);
        }
    }

    let winner = if d1.len() > 0 { 1 } else { 2 };
    let mut winning_stack = if d1.len() > 0 { d1 } else { d2 };

    let mut score = 0;
    // Score is calculated as follows:
    // The bottom card is worth its value * 1
    // The next card is worth its value * 2
    // etc.
    winning_stack.reverse(); // Start from the bottom
    for i in 0..winning_stack.len() {
        score += winning_stack[i] * (i + 1);
    }

    println!("Puzzle 1: player {} wins, with score: {}", winner, score);
}
pub fn puzzle2() {
    let (d1, d2) = match read_decks() {
        Err(e) => return eprintln!("{}", e),
        Ok(v) => v,
    };

    // let d1 = vec![9, 2, 6, 3, 1];
    // let d2 = vec![5, 8, 4, 7, 10];

    // this time, we'll play the game in a recursive fashion.
    // Such complicated rules, but here we go:
    // If, within the _same_ game, both d1 and d2 are in a combination seen before, p1 wins.
    // Otherwise, grab the top two cards (v1 and v2)
    // - If p1 has at least v1 cards, and p2 has at least v2 cards:
    //   - The winner is determined by running a sub-game, cloning v1 cards from d1, and v2 cards from d2
    // - Otherwise, the highest card wins.

    fn play_game(d1: Vec<usize>, d2: Vec<usize>, max_game_id: &mut usize) -> (u32, Vec<usize>) {
        let mut seen_states = vec![];

        let mut p1_cards = d1.clone();
        let mut p2_cards = d2.clone();

        max_game_id.add_assign(1);
        let game_id = max_game_id.clone();
        println!("Playing game {}", game_id);

        let mut round = 1;

        loop {
            // Winning condition: if either player has no cards left, the other player wins.
            if p1_cards.len() == 0 {
                println!("Player 2 wins game {}", game_id);
                return (2, p2_cards);
            }
            if p2_cards.len() == 0 {
                println!("Player 1 wins game {}", game_id);
                return (1, p1_cards);
            }

            // get current state:
            let state = format!(
                "P1({});;P2({})",
                p1_cards
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
                p2_cards
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(";")
            );
            if seen_states.contains(&state) {
                // Winning condition, this exact game state has already been played.
                println!("Already seen state! Player 1 wins game {}", game_id);
                return (1, p1_cards);
            }
            seen_states.push(state);

            // Grab the top two cards:
            let p1_card = p1_cards[0];
            p1_cards = p1_cards[1..].to_vec();
            let p2_card = p2_cards[0];
            p2_cards = p2_cards[1..].to_vec();

            if p1_cards.len() >= p1_card && p2_cards.len() >= p2_card {
                println!(
                    "Playing a recursize sub-game for round {} of game {}",
                    round, game_id
                );

                // Both players have enough cards in their pile to play a recursive game!
                let (winner, _) = play_game(
                    p1_cards[0..p1_card].to_vec(),
                    p2_cards[0..p2_card].to_vec(),
                    max_game_id,
                );

                if winner == 1 {
                    println!(
                        "Player 1 won the sub-game in round {} of game {}",
                        round, game_id
                    );
                    p1_cards.push(p1_card);
                    p1_cards.push(p2_card);
                } else {
                    println!(
                        "Player 2 won the sub-game in round {} of game {}",
                        round, game_id
                    );
                    p2_cards.push(p2_card);
                    p2_cards.push(p1_card);
                }
            } else {
                // Not enough cards, highest wins
                if p1_card > p2_card {
                    println!("Player 1 won round {} of game {}", round, game_id);
                    p1_cards.push(p1_card);
                    p1_cards.push(p2_card);
                } else {
                    println!("Player 2 won round {} of game {}", round, game_id);
                    p2_cards.push(p2_card);
                    p2_cards.push(p1_card);
                }
            }

            round += 1;
        }
    }

    let mut max_game_id = 0;
    let (winner, mut winning_stack) = play_game(d1, d2, &mut max_game_id);

    let mut score = 0;
    winning_stack.reverse(); // Start from the bottom
    for i in 0..winning_stack.len() {
        score += winning_stack[i] * (i + 1);
    }

    println!(
        "Puzzle 2: player {} wins (after a total of {} games), with score: {}",
        winner, max_game_id, score
    );
}
