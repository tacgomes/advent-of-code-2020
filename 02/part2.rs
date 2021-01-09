use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct PasswordPolicy {
    pos1: usize,
    pos2: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(pos1: usize, pos2: usize, letter: char) -> Self {
        PasswordPolicy { pos1, pos2, letter }
    }

    fn validate_password(&self, password: &str) -> bool {
        let match1 = password.chars().nth(self.pos1 - 1) == Some(self.letter);
        let match2 = password.chars().nth(self.pos2 - 1) == Some(self.letter);
        (match1 || match2) && !(match1 && match2)
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<(PasswordPolicy, String)> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|x| {
            let mut tokens = x.split_whitespace();
            let mut pos_tokens = tokens.next().unwrap().split('-');
            let policy = PasswordPolicy::new(
                pos_tokens.next().unwrap().parse::<usize>().unwrap(),
                pos_tokens.next().unwrap().parse::<usize>().unwrap(),
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
        assert_eq!(count_valid_passwords(&passwords), 1);
    }

    #[test]
    fn test_puzzle_input() {
        let passwords = parse_input("input.txt");
        assert_eq!(count_valid_passwords(&passwords), 747);
    }
}
