use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(min: usize, max: usize, letter: char) -> PasswordPolicy {
        PasswordPolicy { min, max, letter }
    }

    fn is_valid_password(&self, password: &str) -> bool {
        let letter_count = password.matches(self.letter).count();
        letter_count >= self.min && letter_count <= self.max
    }
}

fn main() {
    let input = env::args().nth(1).unwrap();
    let file = File::open(input).unwrap();
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

        if policy.is_valid_password(&tokens[2]) {
            num_valid_passwords += 1;
        }
    }

    println!("Result: {}", num_valid_passwords);
}