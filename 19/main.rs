use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

enum Rule {
    Terminal(char),
    MatchAll(Vec<usize>),
    MatchEither(Vec<usize>, Vec<usize>),
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

    match rules.get(&queue[0]).unwrap() {
        Rule::Terminal(t) => input.starts_with(*t) && matches(&queue[1..], &input[1..], rules),
        Rule::MatchAll(c) => matches(&merge(c, &queue[1..]), input, rules),
        Rule::MatchEither(a, b) => {
            matches(&merge(a, &queue[1..]), input, rules)
                || matches(&merge(b, &queue[1..]), input, rules)
        }
    }
}

fn parse_choice(choice: &str) -> Vec<usize> {
    choice
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let mut parts = line.split(':');
    let rule_num = parts.next().unwrap().parse().unwrap();
    let rule_rhs = parts.next().unwrap().trim();

    if rule_rhs.trim().starts_with('"') {
        (rule_num, Rule::Terminal(rule_rhs.chars().nth(1).unwrap()))
    } else {
        let choices = rule_rhs
            .split('|')
            .map(|x| parse_choice(x))
            .collect::<Vec<_>>();
        let rule = match choices.len() {
            1 => Rule::MatchAll(choices[0].clone()),
            2 => Rule::MatchEither(choices[0].clone(), choices[1].clone()),
            _ => panic!(),
        };
        (rule_num, rule)
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> (HashMap<usize, Rule>, Vec<String>) {
    let content = fs::read_to_string(file_name).unwrap();
    let mut blocks = content.split("\n\n");

    let rules = blocks
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| parse_rule(&x))
        .collect();

    let strings = blocks
        .next()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    (rules, strings)
}

fn count_valid_strings(file_name: impl AsRef<Path>) -> usize {
    let (rules, strings) = parse_input(file_name);
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
