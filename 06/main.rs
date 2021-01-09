use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn char_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn count_group_answers_part1(group_answers: &str) -> usize {
    let mut results = [false; 26];

    group_answers
        .chars()
        .filter(|c| !c.is_whitespace())
        .for_each(|answer| results[char_index(answer)] = true);

    results.iter().filter(|&&r| r).count()
}

fn count_group_answers_part2(group_answers: &str) -> usize {
    let mut results = [0; 26];
    let mut group_size = 0;

    for person_answers in group_answers.lines() {
        person_answers
            .chars()
            .for_each(|answer| results[char_index(answer)] += 1);
        group_size += 1;
    }

    results.iter().filter(|&&r| r == group_size).count()
}

fn count_answered_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let group_answers = content.split("\n\n");
    group_answers.map(|g| count_group_answers_part1(g)).sum()
}

fn count_answered_part2(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let group_answers = content.split("\n\n");
    group_answers.map(|g| count_group_answers_part2(g)).sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = count_answered_part1(env::args().nth(1).unwrap());
    let part2 = count_answered_part2(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(count_answered_part1("example.txt"), 11);
        assert_eq!(count_answered_part2("example.txt"), 6);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(count_answered_part1("input.txt"), 6534);
        assert_eq!(count_answered_part2("input.txt"), 3402);
    }
}
