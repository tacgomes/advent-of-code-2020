use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Eq, Hash)]
struct MapPosition {
    r: usize,
    c: usize,
}

impl MapPosition {
    fn new(r: usize, c: usize) -> MapPosition {
        MapPosition { r, c }
    }
}

struct Map {
    num_rows: usize,
    num_cols: usize,
    trees: HashSet<MapPosition>,
}

impl Map {
    fn new(num_rows: usize, num_cols: usize) -> Map {
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
        let mut num_trees = 0;
        let mut current_pos = MapPosition::new(0, 0);

        while current_pos.r != self.num_rows {
            current_pos.r += 1;
            current_pos.c = (current_pos.c + 3) % self.num_cols;

            if self.trees.contains(&current_pos) {
                num_trees += 1;
            }
        }

        num_trees
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();

    let lines = BufReader::new(f).lines();
    let lines: Vec<_> = lines.map(|x| x.unwrap()).collect();

    let mut map = Map::new(lines.len(), lines[0].chars().count());

    for (r, line) in lines.iter().enumerate() {
        for (c, character) in line.chars().enumerate() {
            if character == '#' {
                map.add_tree(MapPosition::new(r, c));
            }
        }
    }

    println!("Result: {}", map.navigate_toboggan());
}
