use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn validate_passport(passport: &str) -> bool {
    let mut fields_found = HashSet::new();

    for field in passport.split_whitespace() {
        if let Some(key) = field.splitn(2, ':').next() {
            if REQUIRED_FIELDS.contains(&key) {
                fields_found.insert(key);
            }
        };
    }

    fields_found.len() == REQUIRED_FIELDS.len()
}

fn valid_passports_count(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let passports = content.split("\n\n");
    passports.filter(|p| validate_passport(p)).count()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = valid_passports_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(valid_passports_count("example.txt"), 2);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(valid_passports_count("input.txt"), 202);
    }
}
