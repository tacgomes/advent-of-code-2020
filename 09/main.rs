use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

fn abs_diff(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn find_encoding_error(
    file_name: impl AsRef<Path>,
    preamble_size: usize,
) -> (Option<u32>, Vec<u32>) {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut numbers = vec![];
    let mut preamble: HashSet<u32> = HashSet::new();

    for (i, line) in lines.enumerate() {
        let num = line.unwrap().parse::<u32>().unwrap();

        if preamble.len() >= preamble_size {
            if !preamble
                .iter()
                .map(|x| abs_diff(num, *x))
                .any(|x| x != num && preamble.contains(&x))
            {
                return (Some(num), numbers);
            }
            preamble.remove(&numbers[i - preamble_size]);
        }

        numbers.push(num);
        preamble.insert(num);
    }

    (None, numbers)
}

fn find_encryption_weakness(error_num: Option<u32>, numbers: &[u32]) -> Option<u32> {
    let error_num = error_num?;

    for sequence_size in 2..numbers.len() {
        for index in 0..numbers.len() - sequence_size {
            let slice = &numbers[index..index + sequence_size];
            if slice.iter().sum::<u32>() == error_num {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return Some(min + max);
            }
        }
    }

    None
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE PREAMBLE-SIZE", env::args().next().unwrap());
        process::exit(1);
    }

    let file_name = env::args().nth(1).unwrap();
    let preamble_size = env::args().nth(2).unwrap().parse::<usize>().unwrap();

    let (error_num, numbers) = find_encoding_error(file_name, preamble_size);
    let encryption_weakness = find_encryption_weakness(error_num, &numbers);
    println!("Result (Part 1): {:?}", error_num);
    println!("Result (Part 2): {:?}", encryption_weakness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let (error_num, numbers) = find_encoding_error("example.txt", 5);
        let encryption_weakness = find_encryption_weakness(error_num, &numbers);
        assert_eq!(error_num, Some(127));
        assert_eq!(encryption_weakness, Some(62));
    }

    #[test]
    fn test_puzzle_input() {
        let (error_num, numbers) = find_encoding_error("input.txt", 25);
        let encryption_weakness = find_encryption_weakness(error_num, &numbers);
        assert_eq!(error_num, Some(57195069));
        assert_eq!(encryption_weakness, Some(7409241));
    }
}
