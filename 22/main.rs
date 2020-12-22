use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::iter::FromIterator;

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let mut player1 = blocks[0].trim().split('\n').skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();
    let mut player2 = blocks[1].trim().split('\n').skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();
    
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    
    let winner = if !player1.is_empty() { player1 } else { player2 };
    winner.iter().rev().enumerate().map(|(i, v)| v * (i + 1)).sum()
}

fn play_recursive(player1: &mut VecDeque::<usize>, player2: &mut VecDeque::<usize>) -> usize {
    println!("NEW GAME");
    let mut played = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
    
        if played.contains(&(p1, p2)) {
            return 0;
        } else if player1.len() >= p1 && player2.len() >= p2 {
            //let copy1 = player1.clone();
            //let copy2 = player2.clone();
            //let winner = play_recursive(&copy1
            // let copy1 = VecDeque::from_iter(player1[0..p1].iter().cloned());
            //let copy1 = VecDeque::from_iter(&player1[0..5].iter().cloned());
            //
            let mut copy1 = player1.iter().take(p1).cloned().collect::<VecDeque<_>>();
            let mut copy2 = player2.iter().take(p2).cloned().collect::<VecDeque<_>>();
            let winner = play_recursive(&mut copy1, &mut copy2);
            if winner == 0 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        } else {
            if p1 > p2 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        }
        played.insert((p1, p2));
    }

    if !player1.is_empty() { 0 } else { 1}
}

fn calculate_part2(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let mut player1 = blocks[0].trim().split('\n').skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();
    let mut player2 = blocks[1].trim().split('\n').skip(1).map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<usize>>();

    play_recursive(&mut player1, &mut player2);
    let winner = if !player1.is_empty() { player1 } else { player2 };
    println!("Winner's deck: {:?}", winner);
    winner.iter().rev().enumerate().map(|(i, v)| v * (i + 1)).sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = calculate_part1(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    let part2 = calculate_part2(env::args().nth(1).unwrap());
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_part1("example1.txt"), 71);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 22000);
    }
}
