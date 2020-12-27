use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const NEIGHBORS: [(isize, isize); 6] = [(-1, 1), (1, 1), (2, 0), (1, -1), (-1, -1), (-2, 0)];

fn get_black_tiles(file_name: impl AsRef<Path>) -> HashSet<(isize, isize)> {
    let content = fs::read_to_string(file_name).unwrap();
    let mut black_tiles = HashSet::new();

    for line in content.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            match c {
                'e' => x += 2,
                'w' => x -= 2,
                'n' => {
                    y += 1;
                    match iter.next().unwrap() {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => unreachable!(),
                    }
                }
                's' => {
                    y -= 1;
                    match iter.next().unwrap() {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        if !black_tiles.remove(&(x, y)) {
            black_tiles.insert((x, y));
        }
    }

    black_tiles
}

fn caculate_part2_black_tiles_count(mut black_tiles: HashSet<(isize, isize)>) -> usize {
    for _ in 0..100 {
        let mut new_black_tiles = HashSet::new();
        let mut white_tiles = HashSet::new();

        for (x, y) in &black_tiles {
            let mut num_black_tiles = 0;
            for neigh in &NEIGHBORS {
                let (neigh_x, neigh_y) = (x + neigh.0, y + neigh.1);
                if black_tiles.contains(&(neigh_x, neigh_y)) {
                    num_black_tiles += 1;
                } else {
                    white_tiles.insert((neigh_x, neigh_y));
                }
            }
            if !(num_black_tiles == 0 || num_black_tiles > 2) {
                new_black_tiles.insert((*x, *y));
            }
        }

        for (x, y) in &white_tiles {
            let mut num_black_tiles = 0;
            for neigh in &NEIGHBORS {
                let (neigh_x, neigh_y) = (x + neigh.0, y + neigh.1);
                if black_tiles.contains(&(neigh_x, neigh_y)) {
                    num_black_tiles += 1;
                }
            }
            if num_black_tiles == 2 {
                new_black_tiles.insert((*x, *y));
            }
        }

        black_tiles = new_black_tiles;
    }

    black_tiles.len()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let black_tiles = get_black_tiles(env::args().nth(1).unwrap());
    let part1_count = black_tiles.len();
    let part2_count = caculate_part2_black_tiles_count(black_tiles);
    println!("Result (Part 1): {}", part1_count);
    println!("Result (Part 2): {}", part2_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let black_tiles = get_black_tiles("example.txt");
        assert_eq!(black_tiles.len(), 10);
        assert_eq!(caculate_part2_black_tiles_count(black_tiles), 2208);
    }

    #[test]
    fn test_puzzle_input() {
        let black_tiles = get_black_tiles("input.txt");
        assert_eq!(black_tiles.len(), 512);
        assert_eq!(caculate_part2_black_tiles_count(black_tiles), 4120);
    }
}
