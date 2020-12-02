use std::fs::File;
use std::io::{prelude::*, BufReader};

struct PasswordPolicy {
    pos1: usize,
    pos2: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(pos1: usize, pos2: usize, letter: char) -> PasswordPolicy {
        PasswordPolicy { pos1, pos2, letter }
    }
}

fn valid_password(policy: &PasswordPolicy, password: &str) -> bool {
    let match_pos1 = password.chars().nth(policy.pos1 - 1).unwrap() == policy.letter;
    let match_pos2 = password.chars().nth(policy.pos2 - 1).unwrap() == policy.letter;
    (match_pos1 || match_pos2) && !(match_pos1 && match_pos2)
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut num_valid_passwords = 0;

    for line in lines {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let pos_tokens: Vec<_> = tokens[0].split('-').collect();
        let policy = PasswordPolicy::new(
            pos_tokens[0].parse::<usize>().unwrap(),
            pos_tokens[1].parse::<usize>().unwrap(),
            tokens[1].chars().next().unwrap(),
        );

        if valid_password(&policy, &tokens[2]) {
            num_valid_passwords += 1;
        }
    }

    println!("Result: {}", num_valid_passwords);
}
