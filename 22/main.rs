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

fn calculate_score(deck1: &Deck, deck2: &Deck) -> usize {
    let winner_hand = if !deck1.is_empty() { deck1 } else { deck2 };
    winner_hand
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + v * (i + 1))
}

fn combat_score(deck1: &mut Deck, deck2: &mut Deck) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    calculate_score(deck1, deck2)
}

fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    let mut played_hands = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        if !played_hands.insert((deck1.clone(), deck2.clone())) {
            return Player::One;
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if deck1.len() >= card1 && deck2.len() >= card2 {
            let mut subdeck1 = deck1.iter().take(card1).cloned().collect();
            let mut subdeck2 = deck2.iter().take(card2).cloned().collect();
            match recursive_combat(&mut subdeck1, &mut subdeck2) {
                Player::One => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                Player::Two => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if !deck1.is_empty() {
        Player::One
    } else {
        Player::Two
    }
}

fn recursive_combat_score(deck1: &mut Deck, deck2: &mut Deck) -> usize {
    recursive_combat(deck1, deck2);
    calculate_score(deck1, deck2)
}

fn parse_input(file_name: impl AsRef<Path>) -> (Deck, Deck) {
    let content = fs::read_to_string(file_name).unwrap();
    let mut blocks = content.split("\n\n");

    let deck1 = blocks
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let deck2 = blocks
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    (deck1, deck2)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (mut deck1, mut deck2) = parse_input(env::args().nth(1).unwrap());
    let part1 = combat_score(&mut deck1.clone(), &mut deck2.clone());
    let part2 = recursive_combat_score(&mut deck1, &mut deck2);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let (deck1, deck2) = parse_input("example.txt");
        assert_eq!(combat_score(&mut deck1.clone(), &mut deck2.clone()), 306);
        assert_eq!(
            recursive_combat_score(&mut deck1.clone(), &mut deck2.clone()),
            291
        );
    }

    #[test]
    fn test_puzzle_input_combat() {
        let (mut deck1, mut deck2) = parse_input("input.txt");
        assert_eq!(combat_score(&mut deck1, &mut deck2), 31957);
    }

    #[test]
    fn test_puzzle_input_recursive_combat() {
        let (mut deck1, mut deck2) = parse_input("input.txt");
        assert_eq!(recursive_combat_score(&mut deck1, &mut deck2), 33212);
    }
}
