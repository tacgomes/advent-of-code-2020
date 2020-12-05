use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

const TARGET_SUM: i32 = 2020;

fn find_solution(file_name: impl AsRef<Path>) -> Option<i32> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut vec = vec![];
    let mut set = HashSet::new();

    for line in lines {
        let n = line.unwrap().parse::<i32>().unwrap();
        vec.push(n);
        set.insert(n);
    }

    // Assumes non-repeated elements
    assert_eq!(set.len(), vec.len());

    for (a_i, a) in vec[..vec.len() - 2].iter().enumerate() {
        for b in vec[a_i + 1..].iter() {
            let diff = TARGET_SUM - a - b;
            if set.contains(&diff) {
                return Some(a * b * diff);
            }
        }
    }
    None
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let result = find_solution(env::args().nth(1).unwrap());
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(find_solution("example.txt"), Some(241861950));
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(find_solution("input.txt"), Some(23869440));
    }
}
