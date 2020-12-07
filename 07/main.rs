use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

use regex::Regex;

const BAG: &str = "shiny gold";

struct BagsManager {
    outer_bags_map: HashMap<String, Vec<String>>,
    inner_bags_count: HashMap<String, HashMap<String, usize>>,
}

impl BagsManager {
    fn new(file_name: impl AsRef<Path>) -> Self {
        let file = File::open(file_name).unwrap();
        let lines = BufReader::new(file).lines();

        let re1 = Regex::new(r"(?P<bag>.+) bags contain (?P<inner_bags>.+)\.").unwrap();
        let re2 = Regex::new(r"(?P<count>\d+) (?P<inner_bag>.+?) bags?").unwrap();

        let mut outer_bags_map = HashMap::new();
        let mut inner_bags_count = HashMap::new();

        for line in lines {
            let line = line.unwrap();
            let caps = re1.captures(&line).unwrap();
            let (bag, inner_bags) = (caps["bag"].to_string(), &caps["inner_bags"]);
            for cap in re2.captures_iter(&inner_bags) {
                let (count, inner_bag) =
                    (cap["count"].parse().unwrap(), cap["inner_bag"].to_string());
                outer_bags_map
                    .entry(inner_bag.clone())
                    .or_insert_with(Vec::new)
                    .push(bag.clone());
                inner_bags_count
                    .entry(bag.clone())
                    .or_insert_with(HashMap::new)
                    .insert(inner_bag, count);
            }
        }

        BagsManager {
            outer_bags_map,
            inner_bags_count,
        }
    }

    fn bag_colors_count(&self, bag: &str) -> usize {
        let mut bag_set = HashSet::new();
        self.bag_colors_count_util(bag, &mut bag_set);
        bag_set.len()
    }

    fn bag_colors_count_util(&self, bag: &str, mut bag_set: &mut HashSet<String>) {
        if let Some(outer_bags) = self.outer_bags_map.get(&bag.to_string()) {
            for outer_bag in outer_bags {
                bag_set.insert(outer_bag.to_string());
                self.bag_colors_count_util(&outer_bag, &mut bag_set);
            }
        }
    }

    fn bags_required(&self, bag: &str) -> usize {
        self.inner_bags_count
            .get(&bag.to_string())
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(inner_bag, count)| count + count * self.bags_required(inner_bag))
            .sum()
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let bags_manager = BagsManager::new(env::args().nth(1).unwrap());
    let colors_count = bags_manager.bag_colors_count(BAG);
    let bags_required = bags_manager.bags_required(BAG);
    println!("Result (Part 1): {}", colors_count);
    println!("Result (Part 2): {}", bags_required);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let bags_manager = BagsManager::new("example1.txt");
        assert_eq!(bags_manager.bag_colors_count(BAG), 4);
        assert_eq!(bags_manager.bags_required(BAG), 32);
    }

    #[test]
    fn test_example_input_2() {
        let bags_manager = BagsManager::new("example2.txt");
        assert_eq!(bags_manager.bag_colors_count(BAG), 0);
        assert_eq!(bags_manager.bags_required(BAG), 126);
    }

    #[test]
    fn test_puzzle_input() {
        let bags_manager = BagsManager::new("input.txt");
        assert_eq!(bags_manager.bag_colors_count(BAG), 259);
        assert_eq!(bags_manager.bags_required(BAG), 45018);
    }
}
