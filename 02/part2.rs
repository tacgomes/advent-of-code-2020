use std::env;
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

    fn is_valid_password(&self, password: &str) -> bool {
        let match_pos1 = password.chars().nth(self.pos1 - 1).unwrap() == self.letter;
        let match_pos2 = password.chars().nth(self.pos2 - 1).unwrap() == self.letter;
        (match_pos1 || match_pos2) && !(match_pos1 && match_pos2)
    }
}

fn main() {
    let input = env::args().nth(1).unwrap();
    let f = File::open(input).unwrap();
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

        if policy.is_valid_password(&tokens[2]) {
            num_valid_passwords += 1;
        }
    }

    println!("Result: {}", num_valid_passwords);
}
