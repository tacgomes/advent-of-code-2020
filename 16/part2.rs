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

fn valid_range(ranges: &[&FieldRange], n: usize) -> bool {
    ranges.iter().any(|r| r.valid_range(n))
}

fn calculate_part2(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let re = Regex::new(r"(?P<f>.+): (?P<s1>\d+)-(?P<e1>\d+) or (?P<s2>\d+)-(?P<e2>\d+)").unwrap();

    let ranges = blocks[0]
        .split('\n')
        .map(|x| {
            let caps = re.captures(&x).unwrap();
            let f = caps["f"].to_string();
            let s1 = caps["s1"].parse::<usize>().unwrap();
            let e1 = caps["e1"].parse::<usize>().unwrap();
            let s2 = caps["s2"].parse::<usize>().unwrap();
            let e2 = caps["e2"].parse::<usize>().unwrap();
            (f, FieldRange(s1, e1, s2, e2))
        })
        .collect::<HashMap<String, FieldRange>>();

    let range_values = ranges.values().collect::<Vec<_>>();

    let my_ticket = blocks[1]
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let nearby_tickets = blocks[2]
        .splitn(2, '\n')
        .nth(1)
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|x| x.iter().all(|&y| valid_range(&range_values, y)))
        .collect::<Vec<_>>();

    let mut position_values = vec![vec![]; nearby_tickets[0].len()];
    for ticket in &nearby_tickets {
        for (index, val) in ticket.iter().enumerate() {
            position_values[index].push(val);
        }
    }

    let mut candidate_positions: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut final_positions: HashMap<String, usize> = HashMap::new();

    for (field, range) in &ranges {
        position_values
            .iter()
            .enumerate()
            .filter(|(_, values)| values.iter().all(|&&v| range.valid_range(v)))
            .for_each(|(index, _)| {
                candidate_positions
                    .entry(field.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(index);
            });
    }

    while final_positions.len() != position_values.len() {
        let candidate = candidate_positions
            .iter()
            .find(|(_, positions)| positions.len() == 1)
            .unwrap();
        let field = candidate.0.clone();
        let position = *candidate.1.iter().next().unwrap();
        candidate_positions.remove(&field);
        final_positions.insert(field.clone(), position);
        for v in candidate_positions.values_mut() {
            v.remove(&position);
        }
    }

    ranges
        .keys()
        .filter(|k| k.starts_with("departure"))
        .map(|k| my_ticket[*final_positions.get(k).unwrap()])
        .product()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part2 = calculate_part2(env::args().nth(1).unwrap());
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_2() {
        assert_eq!(calculate_part2("example2.txt"), 1);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part2("input.txt"), 410460648673);
    }
}