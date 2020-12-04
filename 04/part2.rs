use std::env;
use std::fs;

fn validate_passport(passport: &str) -> bool {
    let mut field_count = 0;

    for field in passport.split_whitespace() {
        let parts: Vec<_> = field.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let (key, value) = (parts[0], parts[1]);

        match key {
            "byr" => {
                if validate_byr(value) {
                    field_count += 1;
                }
            }
            "iyr" => {
                if validate_iyr(value) {
                    field_count += 1;
                }
            }
            "eyr" => {
                if validate_eyr(value) {
                    field_count += 1;
                }
            }
            "hgt" => {
                if validate_hgt(value) {
                    field_count += 1;
                }
            }
            "hcl" => {
                if validate_hcl(value) {
                    field_count += 1;
                }
            }
            "ecl" => {
                if validate_ecl(value) {
                    field_count += 1;
                }
            }
            "pid" => {
                if validate_pid(value) {
                    field_count += 1;
                }
            }
            _ => {}
        }
    }

    field_count == 7
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

    let num = match &value[..value.len() - 2].parse::<u32>() {
        Ok(num) => *num,
        Err(_) => return false,
    };

    let unit = &value[value.len() - 2..];

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

    if value.chars().next().unwrap() != '#' {
        return false;
    }

    for c in value.chars().skip(1) {
        match c {
            '0'..='9' => continue,
            'a'..='f' => continue,
            _ => return false,
        }
    }

    true
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    value.chars().count() == 9 && value.chars().all(char::is_numeric)
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let content = fs::read_to_string(input).unwrap();
    let passports = content.split("\n\n");

    let num_valid = passports.filter(|p| validate_passport(p)).count();
    println!("Result: {}", num_valid);
}
