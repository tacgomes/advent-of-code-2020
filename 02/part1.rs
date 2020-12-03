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
}

fn valid_password(policy: &PasswordPolicy, password: &str) -> bool {
    let letter_count = password.matches(policy.letter).count();
    letter_count >= policy.min && letter_count <= policy.max
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

        if valid_password(&policy, &tokens[2]) {
            num_valid_passwords += 1;
        }
    }

    println!("Result: {}", num_valid_passwords);
}
