use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(min: usize, max: usize, letter: char) -> Self {
        PasswordPolicy { min, max, letter }
    }

    fn validate_password(&self, password: &str) -> bool {
        let letter_count = password.matches(self.letter).count();
        letter_count >= self.min && letter_count <= self.max
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<(PasswordPolicy, String)> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|x| {
            let mut tokens = x.split_whitespace();
            let mut range_tokens = tokens.next().unwrap().split('-');
            let policy = PasswordPolicy::new(
                range_tokens.next().unwrap().parse::<usize>().unwrap(),
                range_tokens.next().unwrap().parse::<usize>().unwrap(),
                tokens.next().unwrap().chars().next().unwrap(),
            );
            (policy, tokens.next().unwrap().to_owned())
        })
        .collect()
}

fn count_valid_passwords(passwords: &[(PasswordPolicy, String)]) -> usize {
    passwords
        .iter()
        .filter(|(pol, pass)| pol.validate_password(&pass))
        .count()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let passwords = parse_input(env::args().nth(1).unwrap());
    let count = count_valid_passwords(&passwords);
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let passwords = parse_input("example.txt");
        assert_eq!(count_valid_passwords(&passwords), 2);
    }

    #[test]
    fn test_puzzle_input() {
        let passwords = parse_input("input.txt");
        assert_eq!(count_valid_passwords(&passwords), 424);
    }
}
