use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use regex::Regex;

const BAG: &str = "shiny gold";

type OuterBags = HashMap<String, Vec<String>>;
type InnerBagsCount = HashMap<String, HashMap<String, usize>>;

struct Bags {
    outer_bags: OuterBags,
    inner_bags_count: InnerBagsCount,
}

impl Bags {
    fn new(outer_bags: OuterBags, inner_bags_count: InnerBagsCount) -> Self {
        Bags {
            outer_bags,
            inner_bags_count,
        }
    }

    fn count_bag_colors(&self, bag: &str) -> usize {
        let mut bags = HashSet::new();
        self.count_bag_colors_util(bag, &mut bags);
        bags.len()
    }

    fn count_bag_colors_util(&self, bag: &str, mut bags: &mut HashSet<String>) {
        if let Some(outer_bags) = self.outer_bags.get(&bag.to_owned()) {
            for outer_bag in outer_bags {
                bags.insert(outer_bag.to_owned());
                self.count_bag_colors_util(&outer_bag, &mut bags);
            }
        }
    }

    fn count_bags_required(&self, bag: &str) -> usize {
        self.inner_bags_count
            .get(&bag.to_owned())
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(inner_bag, count)| count + count * self.count_bags_required(inner_bag))
            .sum()
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> (OuterBags, InnerBagsCount) {
    let re1 = Regex::new(r"(?P<bag>.+) bags contain (?P<inner_bags>.+)\.").unwrap();
    let re2 = Regex::new(r"(?P<count>\d+) (?P<inner_bag>.+?) bags?").unwrap();

    let content = fs::read_to_string(&file_name).unwrap();
    let mut outer_bags = HashMap::new();
    let mut inner_bags_count = HashMap::new();

    for line in content.lines() {
        let caps = re1.captures(&line).unwrap();
        let (bag, inner_bags) = (caps["bag"].to_string(), &caps["inner_bags"]);
        for cap in re2.captures_iter(&inner_bags) {
            let (count, inner_bag) = (cap["count"].parse().unwrap(), cap["inner_bag"].to_string());
            outer_bags
                .entry(inner_bag.clone())
                .or_insert_with(Vec::new)
                .push(bag.clone());
            inner_bags_count
                .entry(bag.clone())
                .or_insert_with(HashMap::new)
                .insert(inner_bag, count);
        }
    }

    (outer_bags, inner_bags_count)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (bags, counts) = parse_input(env::args().nth(1).unwrap());
    let bags_manager = Bags::new(bags, counts);
    let part1 = bags_manager.count_bag_colors(BAG);
    let part2 = bags_manager.count_bags_required(BAG);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let (bags, counts) = parse_input("example1.txt");
        let bags_manager = Bags::new(bags, counts);
        assert_eq!(bags_manager.count_bag_colors(BAG), 4);
        assert_eq!(bags_manager.count_bags_required(BAG), 32);
    }

    #[test]
    fn test_example_input_2() {
        let (bags, counts) = parse_input("example2.txt");
        let bags_manager = Bags::new(bags, counts);
        assert_eq!(bags_manager.count_bag_colors(BAG), 0);
        assert_eq!(bags_manager.count_bags_required(BAG), 126);
    }

    #[test]
    fn test_puzzle_input() {
        let (bags, counts) = parse_input("input.txt");
        let bags_manager = Bags::new(bags, counts);
        assert_eq!(bags_manager.count_bag_colors(BAG), 259);
        assert_eq!(bags_manager.count_bags_required(BAG), 45018);
    }
}
