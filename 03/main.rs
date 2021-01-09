use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

type Point = (usize, usize);

struct Map {
    ncols: usize,
    trees: HashSet<Point>,
}

impl Map {
    fn new(ncols: usize, trees: HashSet<Point>) -> Self {
        Map { ncols, trees }
    }

    fn count_trees_part1(&self) -> usize {
        self.count_trees(&(1, 3))
    }

    fn count_trees_part2(&self) -> usize {
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|m| self.count_trees(m))
            .product()
    }

    fn count_trees(&self, mov: &Point) -> usize {
        self.trees
            .iter()
            .filter(|t| t.0 % mov.0 == 0 && t.1 == (t.0 / mov.0 * mov.1) % self.ncols)
            .count()
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Map {
    let content = fs::read_to_string(file_name).unwrap();

    let trees = content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect();

    Map::new(content.lines().next().unwrap().chars().count(), trees)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let map = parse_input(env::args().nth(1).unwrap());
    let part1 = map.count_trees_part1();
    let part2 = map.count_trees_part2();
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let map = parse_input("example.txt");
        assert_eq!(map.count_trees_part1(), 7);
        assert_eq!(map.count_trees_part2(), 336);
    }

    #[test]
    fn test_puzzle_input() {
        let map = parse_input("input.txt");
        assert_eq!(map.count_trees_part1(), 207);
        assert_eq!(map.count_trees_part2(), 2655892800);
    }
}
