use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn find_encoding_error(numbers: &[usize], preamble_length: usize) -> Option<usize> {
    let absdiff = |a: usize, b: usize| if a > b { a - b } else { b - a };
    let mut preamble = HashSet::new();

    for (i, &num) in numbers.iter().enumerate() {
        if preamble.len() >= preamble_length {
            if !preamble
                .iter()
                .map(|x| absdiff(num, *x))
                .any(|x| x != num && preamble.contains(&x))
            {
                return Some(num);
            }
            preamble.remove(&numbers[i - preamble_length]);
        }
        preamble.insert(num);
    }

    None
}

fn find_encryption_weakness(numbers: &[usize], error: usize) -> Option<usize> {
    (2..numbers.len())
        .flat_map(|x| numbers.windows(x))
        .find_map(|w| {
            if error == w.iter().sum() {
                Some(w.iter().min().unwrap() + w.iter().max().unwrap())
            } else {
                None
            }
        })
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<usize> {
    fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE PREAMBLE-SIZE", env::args().next().unwrap());
        process::exit(1);
    }

    let numbers = parse_input(env::args().nth(1).unwrap());
    let preamble_length = env::args().nth(2).unwrap().parse::<usize>().unwrap();
    let error = find_encoding_error(&numbers, preamble_length).unwrap();
    let encryption_weakness = find_encryption_weakness(&numbers, error);
    println!("Result (Part 1): {:?}", error);
    println!("Result (Part 2): {:?}", encryption_weakness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let numbers = parse_input("example.txt");
        let error = find_encoding_error(&numbers, 5);
        let encryption_weakness = find_encryption_weakness(&numbers, error.unwrap());
        assert_eq!(error, Some(127));
        assert_eq!(encryption_weakness, Some(62));
    }

    #[test]
    fn test_puzzle_input() {
        let numbers = parse_input("input.txt");
        let error = find_encoding_error(&numbers, 25);
        let encryption_weakness = find_encryption_weakness(&numbers, error.unwrap());
        assert_eq!(error, Some(57195069));
        assert_eq!(encryption_weakness, Some(7409241));
    }
}
