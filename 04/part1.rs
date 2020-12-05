use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn validate_passport(passport: &str) -> bool {
    let mut field_count = 0;

    for field in passport.split_whitespace() {
        let key = match field.split(':').next() {
            Some(key) => key,
            _ => continue,
        };

        match key {
            "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => {
                field_count += 1;
            }
            _ => continue,
        }
    }

    field_count == 7
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
