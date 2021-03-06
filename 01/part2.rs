use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const TARGET_SUM: i32 = 2020;

fn solve(file_name: impl AsRef<Path>) -> Option<i32> {
    let content = fs::read_to_string(&file_name).unwrap();

    let mut vec = vec![];
    let mut set = HashSet::new();

    for line in content.lines() {
        let n = line.parse::<i32>().unwrap();
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

    let result = solve(env::args().nth(1).unwrap());
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(solve("example.txt"), Some(241861950));
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve("input.txt"), Some(23869440));
    }
}
