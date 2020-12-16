use std::env;
use std::fs;
use std::path::Path;
use std::process;

use regex::Regex;

struct FieldRange(usize, usize, usize, usize);

impl FieldRange {
    fn valid_range(&self, n: usize) -> bool {
        (n >= self.0 && n <= self.1) || (n >= self.2 && n <= self.3)
    }
}

fn valid_range(ranges: &[FieldRange], n: usize) -> bool {
    ranges.iter().any(|r| r.valid_range(n))
}

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let re = Regex::new(r"(?P<f>.+): (?P<s1>\d+)-(?P<e1>\d+) or (?P<s2>\d+)-(?P<e2>\d+)").unwrap();

    let ranges = blocks[0]
        .split('\n')
        .map(|x| {
            let caps = re.captures(&x).unwrap();
            let s1 = caps["s1"].parse::<usize>().unwrap();
            let e1 = caps["e1"].parse::<usize>().unwrap();
            let s2 = caps["s2"].parse::<usize>().unwrap();
            let e2 = caps["e2"].parse::<usize>().unwrap();
            FieldRange(s1, e1, s2, e2)
        })
        .collect::<Vec<_>>();

    blocks[2]
        .splitn(2, '\n')
        .nth(1)
        .unwrap()
        .trim()
        .split(&['\n', ','][..])
        .map(|x| x.parse::<usize>().unwrap())
        .filter(|&x| !valid_range(&ranges, x))
        .sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = calculate_part1(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_part1("example1.txt"), 71);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 22000);
    }
}
