use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const TARGET_SUM: i32 = 2020;

fn solve_part1(file_name: impl AsRef<Path>) -> Option<i32> {
    let content = fs::read_to_string(file_name).unwrap();

    let mut set = HashSet::new();

    for line in content.lines() {
        let n = line.parse::<i32>().unwrap();
        let diff = TARGET_SUM - n;
        if set.contains(&diff) {
            return Some(n * diff);
        }
        set.insert(n);
    }

    None
}

fn solve_part2(file_name: impl AsRef<Path>) -> Option<i32> {
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

    let part1 = solve_part1(env::args().nth(1).unwrap());
    let part2 = solve_part2(env::args().nth(1).unwrap());
    println!("Result (Part 1):{:?}", part1);
    println!("Result (Part 2):{:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(solve_part1("example.txt"), Some(514579));
        assert_eq!(solve_part2("example.txt"), Some(241861950));
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve_part1("input.txt"), Some(918339));
        assert_eq!(solve_part2("input.txt"), Some(23869440));
    }
}
