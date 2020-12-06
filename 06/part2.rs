use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn group_answers_count(group_answers: &str) -> usize {
    let mut results = [0; 26];
    let mut group_size = 0;

    for person_answers in group_answers.lines() {
        for answer in person_answers.chars() {
            results[answer as usize - 'a' as usize] += 1;
        }
        group_size += 1;
    }

    results.iter().filter(|&&r| r == group_size).count()
}

fn answered_questions_count(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let group_answers = content.split("\n\n");
    group_answers.map(|g| group_answers_count(g)).sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = answered_questions_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(answered_questions_count("example.txt"), 6);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(answered_questions_count("input.txt"), 3402);
    }
}
