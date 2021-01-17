use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const NEIGHBORS: [(isize, isize); 6] = [(-1, 1), (1, 1), (2, 0), (1, -1), (-1, -1), (-2, 0)];

type Tiles = HashSet<(isize, isize)>;

fn iterate_once(tiles: Tiles) -> Tiles {
    let mut counter = HashMap::new();

    tiles
        .iter()
        .flat_map(|(x, y)| {
            NEIGHBORS
                .iter()
                .map(move |neigh| (x + neigh.0, y + neigh.1))
        })
        .for_each(|(x, y)| *counter.entry((x, y)).or_insert(0) += 1);

    counter
        .iter()
        .filter(|&(k, &c)| (c == 2 || (c == 1 && tiles.contains(&k))))
        .map(|(k, _)| k)
        .cloned()
        .collect()
}

fn iterate(mut tiles: Tiles) -> Tiles {
    for _ in 0..100 {
        tiles = iterate_once(tiles);
    }
    tiles
}

fn parse_input(file_name: impl AsRef<Path>) -> Tiles {
    let content = fs::read_to_string(file_name).unwrap();
    let mut tiles = HashSet::new();

    /*
     * Use double coordinates to represent the hexagonal grid as
     * explained here:
     * https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
     */
    for line in content.lines() {
        let (mut x, mut y, mut iter) = (0, 0, line.chars());
        while let Some(c) = iter.next() {
            let mut coord = c.to_string();
            if c == 'n' || c == 's' {
                coord.push(iter.next().unwrap());
            }
            match coord.as_str() {
                "nw" => {
                    x -= 1;
                    y += 1;
                }
                "ne" => {
                    x += 1;
                    y += 1;
                }
                "e" => x += 2,
                "se" => {
                    x += 1;
                    y -= 1;
                }
                "sw" => {
                    x -= 1;
                    y -= 1;
                }
                "w" => x -= 2,
                _ => unreachable!(),
            }
        }
        if !tiles.remove(&(x, y)) {
            tiles.insert((x, y));
        }
    }

    tiles
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let tiles1 = parse_input(env::args().nth(1).unwrap());
    let tiles2 = iterate(tiles1.clone());
    let part1 = tiles1.len();
    let part2 = tiles2.len();
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let tiles1 = parse_input("example.txt");
        let tiles2 = iterate(tiles1.clone());
        assert_eq!(tiles1.len(), 10);
        assert_eq!(tiles2.len(), 2208);
    }

    #[test]
    fn test_puzzle_input() {
        let tiles1 = parse_input("input.txt");
        let tiles2 = iterate(tiles1.clone());
        assert_eq!(tiles1.len(), 512);
        assert_eq!(tiles2.len(), 4120);
    }
}
