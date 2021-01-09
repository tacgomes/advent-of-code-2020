use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const TARGET_SUM: i32 = 2020;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<i32> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn solve_part1(values: &[i32]) -> Option<i32> {
    let mut set = HashSet::new();

    for n in values {
        let diff = TARGET_SUM - n;
        if set.contains(&diff) {
            return Some(n * diff);
        }
        set.insert(n);
    }

    None
}

fn solve_part2(values: &[i32]) -> Option<i32> {
    let mut set = HashSet::new();

    for (a_i, a) in values[..values.len() - 2].iter().enumerate() {
        for b in values[a_i + 1..].iter() {
            let c = TARGET_SUM - a - b;
            if set.contains(&c) {
                return Some(a * b * c);
            }
        }
        set.insert(a);
    }

    None
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let values = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&values);
    let part2 = solve_part2(&values);
    println!("Result (Part 1):{:?}", part1);
    println!("Result (Part 2):{:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let values = parse_input("example.txt");
        assert_eq!(solve_part1(&values), Some(514579));
        assert_eq!(solve_part2(&values), Some(241861950));
    }

    #[test]
    fn test_puzzle_input() {
        let values = parse_input("input.txt");
        assert_eq!(solve_part1(&values), Some(918339));
        assert_eq!(solve_part2(&values), Some(23869440));
    }
}
