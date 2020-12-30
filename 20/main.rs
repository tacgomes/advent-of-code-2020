use std::env;
use std::fs;
use std::path::Path;
use std::process;

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

struct Tile {
    tile_id: usize,
    alignments: Vec<Vec<String>>,
    current_alignment: usize,
}

impl Tile {
    fn new(tile_id: usize, borders: Vec<String>) -> Self {
        let mut alignments = vec![];

        // println!("\ntileid: {}", tile_id);
        // println!("borders:  {:?}", borders);

        alignments.push(borders.clone());

        let vertically_mirrored = vec![
            borders[BOTTOM].clone(),
            borders[RIGHT].chars().rev().clone().collect(),
            borders[TOP].clone(),
            borders[LEFT].chars().rev().clone().collect(),
        ];

        // println!("vertical: {:?}", vertically_mirrored);

        alignments.push(vertically_mirrored.clone());

        for border in [&borders, &vertically_mirrored].iter() {
            let mut border = border.to_vec();
            for _ in 0..3 {
                border = vec![
                    border[LEFT].chars().rev().clone().collect(),
                    border[TOP].clone(),
                    border[RIGHT].chars().rev().clone().collect(),
                    border[BOTTOM].clone(),
                ];
                // println!("rotation: {:?}", border);
                alignments.push(border.clone());
            }
        }

        Tile {
            tile_id,
            alignments,
            current_alignment: 0,
        }
    }

    fn next_alignment(&mut self) {
        self.current_alignment += 1;
        self.current_alignment %= self.alignments.len();
    }

    fn top(&self) -> &str {
        return &self.alignments[self.current_alignment][TOP];
    }

    fn right(&self) -> &str {
        return &self.alignments[self.current_alignment][RIGHT];
    }

    fn bottom(&self) -> &str {
        return &self.alignments[self.current_alignment][BOTTOM];
    }

    fn left(&self) -> &str {
        return &self.alignments[self.current_alignment][LEFT];
    }
}

fn find_alignment(_tiles: &mut Vec<Tile>) -> Vec<Vec<&Tile>> {
    let mut _array = vec![];
    _array
}

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.trim().split("\n\n").collect();

    let mut tiles = vec![];

    for tile in blocks {
        let mut lines = tile.split('\n');
        let tile_id = &lines.next().unwrap();
        let tile_id = &tile_id[5..tile_id.len() - 1].parse::<usize>().unwrap();

        let mut borders = vec![String::new(); 4];
        borders[0] = lines.clone().next().unwrap().to_string();
        borders[2] = lines.clone().last().unwrap().to_string();
        for line in lines {
            borders[3].push(line.chars().next().unwrap());
            borders[1].push(line.chars().last().unwrap());
        }

        tiles.push(Tile::new(*tile_id, borders));
    }

    let _array = find_alignment(&mut tiles);
    0
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = calculate_part1(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_input() {
        assert_eq!(calculate_part1("example.txt"), 0);
    }

    #[test]
    #[ignore]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 0);
    }
}
