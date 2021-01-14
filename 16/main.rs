use std::collections::HashMap;
use std::collections::HashSet;
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

type RangeMap = HashMap<String, FieldRange>;

fn valid_range(ranges: &RangeMap, n: usize) -> bool {
    ranges.values().any(|r| r.valid_range(n))
}

fn solve_part1(ranges: &RangeMap, nearby_tickets: &[Vec<usize>]) -> usize {
    nearby_tickets
        .iter()
        .flat_map(|y| y.iter())
        .filter(|&&x| !valid_range(&ranges, x))
        .sum()
}

fn solve_part2(ranges: &RangeMap, ticket: &[usize], nearby_tickets: &[Vec<usize>]) -> usize {
    let nearby_tickets = nearby_tickets
        .iter()
        .filter(|x| x.iter().all(|&y| valid_range(&ranges, y)))
        .collect::<Vec<_>>();

    let mut cols = vec![vec![]; nearby_tickets[0].len()];
    for ticket in &nearby_tickets {
        for (col, val) in ticket.iter().enumerate() {
            cols[col].push(val);
        }
    }

    let mut candidates = HashMap::new();
    let mut positions = HashMap::new();

    for (field, range) in ranges {
        cols
            .iter()
            .enumerate()
            .filter(|(_, values)| values.iter().all(|&&v| range.valid_range(v)))
            .for_each(|(index, _)| {
                candidates
                    .entry(field.clone())
                    .or_insert_with(HashSet::new)
                    .insert(index);
            });
    }

    while positions.len() != cols.len() {
        let candidate = candidates
            .iter()
            .find(|(_, positions)| positions.len() == 1)
            .unwrap();
        let field = candidate.0.clone();
        let position = *candidate.1.iter().next().unwrap();
        candidates.remove(&field);
        positions.insert(field, position);
        for v in candidates.values_mut() {
            v.remove(&position);
        }
    }

    ranges
        .keys()
        .filter(|k| k.starts_with("departure"))
        .map(|k| ticket[*positions.get(k).unwrap()])
        .product()
}

fn parse_input(file_name: impl AsRef<Path>) -> (RangeMap, Vec<usize>, Vec<Vec<usize>>) {
    let content = fs::read_to_string(file_name).unwrap();
    let mut blocks = content.split("\n\n");

    let re = Regex::new(r"(?P<f>.+): (?P<s1>\d+)-(?P<e1>\d+) or (?P<s2>\d+)-(?P<e2>\d+)").unwrap();

    let ranges = blocks
        .next()
        .unwrap()
        .split('\n')
        .map(|x| {
            let caps = re.captures(&x).unwrap();
            let field = caps["f"].to_string();
            let s1 = caps["s1"].parse::<usize>().unwrap();
            let e1 = caps["e1"].parse::<usize>().unwrap();
            let s2 = caps["s2"].parse::<usize>().unwrap();
            let e2 = caps["e2"].parse::<usize>().unwrap();
            (field, FieldRange(s1, e1, s2, e2))
        })
        .collect();

    let ticket = blocks
        .next()
        .unwrap()
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let nearby_tickets = blocks
        .next()
        .unwrap()
        .splitn(2, '\n')
        .nth(1)
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();

    (ranges, ticket, nearby_tickets)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (ranges, ticket, nearby_tickets) = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&ranges, &nearby_tickets);
    let part2 = solve_part2(&ranges, &ticket, &nearby_tickets);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let (ranges, _, nearby_tickets) = parse_input("example1.txt");
        assert_eq!(solve_part1(&ranges, &nearby_tickets), 71);
    }

    #[test]
    fn test_example_input_2() {
        let (ranges, ticket, nearby_tickets) = parse_input("example2.txt");
        assert_eq!(solve_part2(&ranges, &ticket, &nearby_tickets), 1);
    }

    #[test]
    fn test_puzzle_input() {
        let (ranges, ticket, nearby_tickets) = parse_input("input.txt");
        assert_eq!(solve_part1(&ranges, &nearby_tickets), 22000);
        assert_eq!(solve_part2(&ranges, &ticket, &nearby_tickets), 410460648673);
    }
}
