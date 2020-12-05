use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(min: usize, max: usize, letter: char) -> PasswordPolicy {
        PasswordPolicy { min, max, letter }
    }

    fn validate_password(&self, password: &str) -> bool {
        let letter_count = password.matches(self.letter).count();
        letter_count >= self.min && letter_count <= self.max
    }
}

fn valid_passwords_count(file_name: impl AsRef<Path>) -> u32 {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut num_valid_passwords = 0;

    for line in lines {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let range_tokens: Vec<_> = tokens[0].split('-').collect();
        let policy = PasswordPolicy::new(
            range_tokens[0].parse::<usize>().unwrap(),
            range_tokens[1].parse::<usize>().unwrap(),
            tokens[1].chars().next().unwrap(),
        );

        if policy.validate_password(&tokens[2]) {
            num_valid_passwords += 1;
        }
    }
    num_valid_passwords
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = valid_passwords_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(valid_passwords_count("example.txt"), 2);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(valid_passwords_count("input.txt"), 424);
    }
}
