use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Conjunction(Vec<usize>),
    Disjunction(Vec<usize>, Vec<usize>),
}

fn merge(a: &[usize], b: &[usize]) -> Vec<usize> {
    a.iter().chain(b.iter()).cloned().collect()
}

fn matches(queue: &[usize], input: &str, rules: &HashMap<usize, Rule>) -> bool {
    match (queue.is_empty(), input.is_empty()) {
        (true, true) => return true,
        (true, _) => return false,
        (_, true) => return false,
        _ => (),
    }

    let rule = rules.get(&queue[0]).unwrap();

    match rule {
        Rule::Terminal(t) => input.starts_with(*t) && matches(&queue[1..], &input[1..], rules),
        Rule::Conjunction(c) => matches(&merge(c, &queue[1..]), input, rules),
        Rule::Disjunction(a, b) => {
            matches(&merge(a, &queue[1..]), input, rules)
                || matches(&merge(b, &queue[1..]), input, rules)
        }
    }
}

fn parse(file_name: impl AsRef<Path>) -> (HashMap<usize, Rule>, Vec<String>) {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let mut rules = HashMap::new();

    for line in blocks[0].trim().split('\n') {
        let parts = line.split(':').collect::<Vec<_>>();
        let rule_num = parts[0].parse::<usize>().unwrap();
        let rule_rhs = parts[1].trim();

        if rule_rhs.trim().starts_with('"') {
            rules.insert(rule_num, Rule::Terminal(rule_rhs.chars().nth(1).unwrap()));
        } else {
            let disjunctions = rule_rhs
                .split('|')
                .map(|x| {
                    x.split_whitespace()
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let rule = match disjunctions.len() {
                1 => Rule::Conjunction(disjunctions[0].clone()),
                2 => Rule::Disjunction(disjunctions[0].clone(), disjunctions[1].clone()),
                _ => panic!(),
            };

            rules.insert(rule_num, rule);
        }
    }

    let strings = blocks[1].lines().map(|x| x.to_string()).collect();

    (rules, strings)
}

fn count_valid_strings(file_name: impl AsRef<Path>) -> usize {
    let (rules, strings) = parse(file_name);
    strings.iter().filter(|m| matches(&[0], m, &rules)).count()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = count_valid_strings(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(count_valid_strings("example1.txt"), 2);
    }

    #[test]
    fn test_example_input_2_part_1() {
        assert_eq!(count_valid_strings("example2-part1.txt"), 3);
    }

    #[test]
    fn test_example_input_2_part_2() {
        assert_eq!(count_valid_strings("example2-part2.txt"), 12);
    }

    #[test]
    fn test_puzzle_input_1_part_1() {
        assert_eq!(count_valid_strings("input-part1.txt"), 285);
    }

    #[test]
    fn test_puzzle_input_2_part_2() {
        assert_eq!(count_valid_strings("input-part2.txt"), 412);
    }
}
