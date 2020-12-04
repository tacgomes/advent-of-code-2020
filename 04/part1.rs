use std::env;
use std::fs;

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

fn main() {
    let input = env::args().nth(1).unwrap();

    let content = fs::read_to_string(input).unwrap();
    let passports = content.split("\n\n");

    let num_valid = passports.filter(|p| validate_passport(p)).count();
    println!("Result: {}", num_valid);
}