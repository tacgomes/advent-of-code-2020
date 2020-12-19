use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
enum Rule {
    Letter(char),
    Expression(Vec<Vec<usize>>),
}

fn matches(message: &[char], rule_number: usize, rules: &HashMap<usize, Rule>) -> Option<usize> {
    if message.is_empty() {
        return None;
    }

    let rule = rules.get(&rule_number).unwrap();
    match rule {
        Rule::Letter(c) => {
            if message[0] == *c {
                Some(1)
            } else {
                None
            }
        }
        Rule::Expression(or_expressions) => {
            for or_expression in or_expressions {
                let mut total_consumed = 0;
                let mut found_match = true;

                for and_expression in or_expression {
                    let result = matches(&message[total_consumed..], *and_expression, rules);
                    if let Some(consumed) = result {
                        total_consumed += consumed;
                    } else {
                        found_match = false;
                        break;
                    }
                }

                if found_match {
                    return Some(total_consumed);
                }
            }
            None
        }
    }
}

fn calculate_valid_strings_count(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.split("\n\n").collect();

    let mut rules = HashMap::new();

    for line in blocks[0].trim().split('\n') {
        let parts = line.split(':').collect::<Vec<_>>();
        let rule_number = parts[0].parse::<usize>().unwrap();
        let rule_text = parts[1].trim();

        if rule_text.trim().starts_with('"') {
            rules.insert(rule_number, Rule::Letter(rule_text.chars().nth(1).unwrap()));
        } else {
            let mut expression = vec![];
            let mut and_expression = vec![];
            for token in rule_text.split_whitespace() {
                match token {
                    "|" => {
                        expression.push(and_expression.clone());
                        and_expression.clear();
                    }
                    _ => and_expression.push(token.parse::<usize>().unwrap()),
                }
            }
            expression.push(and_expression.clone());
            rules.insert(rule_number, Rule::Expression(expression));
        }
    }

    blocks[1]
        .trim()
        .split('\n')
        .map(|m| m.chars().collect::<Vec<_>>())
        .filter(|m| match matches(&m, 0, &rules) {
            Some(consumed) => m.len() == consumed,
            None => false,
        })
        .count()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = calculate_valid_strings_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_valid_strings_count("example1.txt"), 2);
    }

    // TODO: fix part 2
    #[test]
    #[ignore]
    fn test_example_input_2() {
        assert_eq!(calculate_valid_strings_count("example2.txt"), 12);
    }

    #[test]
    fn test_puzzle_input_1() {
        assert_eq!(calculate_valid_strings_count("input-part1.txt"), 285);
    }

    #[test]
    #[ignore]
    fn test_puzzle_input_2() {
        assert_eq!(calculate_valid_strings_count("input-part2.txt"), 0);
    }
}
