use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

#[derive(PartialEq, Eq, Hash)]
struct MapPosition {
    r: usize,
    c: usize,
}

impl MapPosition {
    fn new(r: usize, c: usize) -> Self {
        MapPosition { r, c }
    }

    fn advance(&mut self, movement: &Self) {
        self.r += movement.r;
        self.c += movement.c;
    }
}

struct Map {
    num_rows: usize,
    num_cols: usize,
    trees: HashSet<MapPosition>,
}

impl Map {
    fn new(num_rows: usize, num_cols: usize) -> Self {
        Map {
            num_rows,
            num_cols,
            trees: HashSet::new(),
        }
    }

    fn add_tree(&mut self, point: MapPosition) {
        self.trees.insert(point);
    }

    fn navigate_toboggan(&self) -> usize {
        let movements = [
            MapPosition::new(1, 1),
            MapPosition::new(1, 3),
            MapPosition::new(1, 5),
            MapPosition::new(1, 7),
            MapPosition::new(2, 1),
        ];

        movements
            .iter()
            .map(|m| self.navigate_toboggan_with_movement(m))
            .product()
    }

    fn navigate_toboggan_with_movement(&self, movement: &MapPosition) -> usize {
        let mut num_trees = 0;
        let mut current_pos = MapPosition::new(0, 0);

        while current_pos.r < self.num_rows {
            current_pos.advance(&movement);
            current_pos.c %= self.num_cols;

            if self.trees.contains(&current_pos) {
                num_trees += 1;
            }
        }

        num_trees
    }
}

fn encountered_trees_count(file_name: impl AsRef<Path>) -> usize {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();
    let lines: Vec<_> = lines.map(|x| x.unwrap()).collect();

    let mut map = Map::new(lines.len(), lines[0].chars().count());

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, ch)| ch == '#')
            .for_each(|(col, _)| map.add_tree(MapPosition::new(row, col)))
    });

    map.navigate_toboggan()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = encountered_trees_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(encountered_trees_count("example.txt"), 336);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(encountered_trees_count("input.txt"), 2655892800);
    }
}
