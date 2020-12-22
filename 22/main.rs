use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

type Deck = VecDeque<usize>;

enum Player {
    One,
    Two,
}

fn read_hands(file_name: impl AsRef<Path>) -> (Deck, Deck) {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let deck1 = blocks[0]
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Deck>();

    let deck2 = blocks[1]
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Deck>();

    (deck1, deck2)
}

fn calculate_score(deck1: &Deck, deck2: &Deck) -> usize {
    let winner_hand = if !deck1.is_empty() { deck1 } else { deck2 };
    winner_hand
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| v * (i + 1))
        .sum()
}

fn combat_score(file_name: impl AsRef<Path>) -> usize {
    let (mut deck1, mut deck2) = read_hands(file_name);

    while !deck1.is_empty() && !deck2.is_empty() {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    calculate_score(&deck1, &deck2)
}

fn combat_subgame(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    let mut played_hands = HashSet::<(String, String)>::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        let deck1_str = deck1
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let deck2_str = deck2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if played_hands.contains(&(deck1_str.clone(), deck2_str.clone())) {
            return Player::One;
        }

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if deck1.len() >= c1 && deck2.len() >= c2 {
            match combat_subgame(&mut deck1.clone(), &mut deck2.clone()) {
                Player::One => {
                    deck1.push_back(c1);
                    deck1.push_back(c2);
                }
                Player::Two => {
                    deck2.push_back(c2);
                    deck2.push_back(c1);
                }
            }
        } else if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
        played_hands.insert((deck1_str, deck2_str));
    }

    if !deck1.is_empty() {
        Player::One
    } else {
        Player::Two
    }
}

fn recursive_combat_score(file_name: impl AsRef<Path>) -> usize {
    let (mut deck1, mut deck2) = read_hands(file_name);
    combat_subgame(&mut deck1, &mut deck2);
    calculate_score(&deck1, &deck2)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = combat_score(env::args().nth(1).unwrap());
    let part2 = recursive_combat_score(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(combat_score("example.txt"), 306);
        assert_eq!(recursive_combat_score("example.txt"), 291);
    }

    #[test]
    fn test_puzzle_input_combat() {
        assert_eq!(combat_score("input.txt"), 31957);
    }

    #[test]
    #[ignore]
    fn test_puzzle_input_recursive_combat() {
        assert_eq!(recursive_combat_score("input.txt"), 33212);
    }
}
