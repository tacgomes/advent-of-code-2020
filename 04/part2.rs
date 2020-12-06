use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

type FnValidator = fn(&str) -> bool;
type Validators<'a> = HashMap<&'a str, FnValidator>;

fn validate_passport(passport: &str, validators: &Validators) -> bool {
    let mut fields_validated = HashSet::new();

    for field in passport.split_whitespace() {
        let parts: Vec<_> = field.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let (key, value) = (parts[0], parts[1]);

        if let Some(validator) = validators.get(key) {
            if validator(value) {
                fields_validated.insert(key);
            }
        }
    }

    fields_validated.len() == validators.len()
}

fn validate_byr(value: &str) -> bool {
    match value.parse::<u32>() {
        Ok(y) => y >= 1920 && y <= 2002,
        Err(_) => false,
    }
}

fn validate_iyr(value: &str) -> bool {
    match value.parse::<u32>() {
        Ok(y) => y >= 2010 && y <= 2020,
        Err(_) => false,
    }
}

fn validate_eyr(value: &str) -> bool {
    match value.parse::<u32>() {
        Ok(y) => y >= 2020 && y <= 2030,
        Err(_) => false,
    }
}

fn validate_hgt(value: &str) -> bool {
    if value.len() < 4 {
        return false;
    }

    let (num, unit) = value.split_at(value.len() - 2);
    let num = match num.parse::<u32>() {
        Ok(num) => num,
        Err(_) => return false,
    };

    match unit {
        "cm" => num >= 150 && num <= 193,
        "in" => num >= 59 && num <= 76,
        _ => false,
    }
}

fn validate_hcl(value: &str) -> bool {
    if value.chars().count() != 7 {
        return false;
    }

    if !value.starts_with('#') {
        return false;
    }

    value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

fn validate_ecl(value: &str) -> bool {
    matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn validate_pid(value: &str) -> bool {
    value.chars().count() == 9 && value.chars().all(char::is_numeric)
}

fn valid_passports_count(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let passports = content.split("\n\n");

    let mut validators: Validators = HashMap::new();
    validators.insert("byr", validate_byr);
    validators.insert("iyr", validate_iyr);
    validators.insert("eyr", validate_eyr);
    validators.insert("hgt", validate_hgt);
    validators.insert("hcl", validate_hcl);
    validators.insert("ecl", validate_ecl);
    validators.insert("pid", validate_pid);

    passports
        .filter(|p| validate_passport(p, &validators))
        .count()
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
        assert_eq!(valid_passports_count("input.txt"), 137);
    }
}
