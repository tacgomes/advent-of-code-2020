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

    fn validate_password_part1(&self, password: &str) -> bool {
        let letter_count = password.matches(self.letter).count();
        letter_count >= self.min && letter_count <= self.max
    }

    fn validate_password_part2(&self, password: &str) -> bool {
        let match1 = password.chars().nth(self.min - 1) == Some(self.letter);
        let match2 = password.chars().nth(self.max - 1) == Some(self.letter);
        (match1 || match2) && !(match1 && match2)
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

fn count_valid_passwords_part1(passwords: &[(PasswordPolicy, String)]) -> usize {
    passwords
        .iter()
        .filter(|(pol, pass)| pol.validate_password_part1(&pass))
        .count()
}

fn count_valid_passwords_part2(passwords: &[(PasswordPolicy, String)]) -> usize {
    passwords
        .iter()
        .filter(|(pol, pass)| pol.validate_password_part2(&pass))
        .count()
}


fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let passwords = parse_input(env::args().nth(1).unwrap());
    let part1 = count_valid_passwords_part1(&passwords);
    let part2 = count_valid_passwords_part2(&passwords);
    println!("Result (Part 1) {}", part1);
    println!("Result (Part 2) {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let passwords = parse_input("example.txt");
        assert_eq!(count_valid_passwords_part1(&passwords), 2);
        assert_eq!(count_valid_passwords_part2(&passwords), 1);
    }

    #[test]
    fn test_puzzle_input() {
        let passwords = parse_input("input.txt");
        assert_eq!(count_valid_passwords_part1(&passwords), 424);
        assert_eq!(count_valid_passwords_part2(&passwords), 747);
    }
}
