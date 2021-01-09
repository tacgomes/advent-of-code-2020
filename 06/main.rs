use std::env;
use std::fs;
use std::path::Path;
use std::process;

const NUM_QUESTIONS: usize = 26;

// NB: another way to tackle this problem would be to use set union and
// set intersection to solve part 1 and part 2. However, as the max size
// of the results is fixed (26), it does not provide advantages.

fn char_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn count_group_answers_part1(group_answers: &str) -> usize {
    let mut results = [false; NUM_QUESTIONS];

    group_answers
        .chars()
        .filter(|c| !c.is_whitespace())
        .for_each(|answer| results[char_index(answer)] = true);

    results.iter().filter(|&&r| r).count()
}

fn count_group_answers_part2(group_answers: &str) -> usize {
    let mut results = [0; NUM_QUESTIONS];

    group_answers
        .lines()
        .flat_map(|x| x.chars())
        .for_each(|answer| results[char_index(answer)] += 1);

    results
        .iter()
        .filter(|&&r| r == group_answers.lines().count())
        .count()
}

fn count_answered_part1(file_name: impl AsRef<Path>) -> usize {
    fs::read_to_string(file_name)
        .unwrap()
        .split("\n\n")
        .map(|x| count_group_answers_part1(x))
        .sum()
}

fn count_answered_part2(file_name: impl AsRef<Path>) -> usize {
    fs::read_to_string(file_name)
        .unwrap()
        .split("\n\n")
        .map(|x| count_group_answers_part2(x))
        .sum()
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
