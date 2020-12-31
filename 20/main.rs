use std::collections::VecDeque;
use std::env;
use std::fs;
use std::mem;
use std::path::Path;
use std::process;

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

type Config = Vec<Vec<Option<Tile>>>;

#[derive(Clone)]
struct Tile {
    tile_id: usize,
    configs: Vec<Vec<String>>,
    current_config: usize,
}

impl Tile {
    fn new(tile_id: usize, borders: Vec<String>) -> Self {
        let mut configs = vec![];

        configs.push(borders.clone());

        let vertically_mirrored = vec![
            borders[BOTTOM].clone(),
            borders[RIGHT].chars().rev().clone().collect(),
            borders[TOP].clone(),
            borders[LEFT].chars().rev().clone().collect(),
        ];

        configs.push(vertically_mirrored.clone());

        for border in [&borders, &vertically_mirrored].iter() {
            let mut border = border.to_vec();
            for _ in 0..3 {
                border = vec![
                    border[LEFT].chars().rev().clone().collect(),
                    border[TOP].clone(),
                    border[RIGHT].chars().rev().clone().collect(),
                    border[BOTTOM].clone(),
                ];
                configs.push(border.clone());
            }
        }

        Tile {
            tile_id,
            configs,
            current_config: 0,
        }
    }

    fn id(&self) -> usize {
        self.tile_id
    }

    fn next_alignment(&mut self) {
        self.current_config += 1;
        self.current_config %= self.configs.len();
    }

    fn top(&self) -> &str {
        &self.configs[self.current_config][TOP]
    }

    fn right(&self) -> &str {
        &self.configs[self.current_config][RIGHT]
    }

    fn bottom(&self) -> &str {
        &self.configs[self.current_config][BOTTOM]
    }

    fn left(&self) -> &str {
        &self.configs[self.current_config][LEFT]
    }
}

fn next_cell(row: usize, col: usize, len: usize) -> (usize, usize) {
    match col + 1 == len {
        true => (row + 1, 0),
        false => (row, col + 1),
    }
}

#[allow(clippy::ptr_arg)]
fn check_config(config: &Config) -> bool {
    let match_vertical = |x: &Option<Tile>, y: &Option<Tile>| {
        y.is_none() || x.as_ref().unwrap().left() == y.as_ref().unwrap().right()
    };
    let match_horizont = |x: &Option<Tile>, y: &Option<Tile>| {
        y.is_none() || x.as_ref().unwrap().bottom() == y.as_ref().unwrap().top()
    };
    config
        .iter()
        .all(|r| r.windows(2).all(|w| match_vertical(&w[0], &w[1])))
        && config
            .windows(2)
            .all(|w| w[0].iter().zip(&w[1]).all(|(x, y)| match_horizont(x, y)))
}

fn find_config(
    mut tiles: &mut VecDeque<Tile>,
    row: usize,
    col: usize,
    mut config: &mut Config,
) -> bool {
    if !check_config(&config) {
        return false;
    }
    if row == config.len() {
        return true;
    }
    let (next_row, next_col) = next_cell(row, col, config.len());

    let mut config_ok = false;
    for _ in 0..tiles.len() {
        let tile = tiles.pop_front().unwrap();
        config[row][col] = Some(tile);
        for _ in 0..8 {
            config_ok = find_config(&mut tiles, next_row, next_col, &mut config);
            if config_ok {
                break;
            }
            config[row][col].as_mut().unwrap().next_alignment();
        }

        match config_ok {
            true => return true,
            false => tiles.push_back(mem::replace(&mut config[row][col], None).unwrap()),
        }
    }

    config[row][col] = None;
    false
}

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks: Vec<_> = content.trim().split("\n\n").collect();

    let mut tiles = VecDeque::new();

    for tile in blocks {
        let mut lines = tile.split('\n');
        let tile_id = &lines.next().unwrap();
        let tile_id = tile_id[5..tile_id.len() - 1].parse::<usize>().unwrap();

        let mut borders = vec![String::new(); 4];
        borders[0] = lines.clone().next().unwrap().to_string();
        borders[2] = lines.clone().last().unwrap().to_string();
        for line in lines {
            borders[3].push(line.chars().next().unwrap());
            borders[1].push(line.chars().last().unwrap());
        }

        tiles.push_back(Tile::new(tile_id, borders));
    }

    let len = (tiles.len() as f64).sqrt() as usize;
    let mut config = vec![vec![None; len]; len];

    let found_config = find_config(&mut tiles, 0, 0, &mut config);
    assert!(found_config);

    [(0, 0), (0, len - 1), (len - 1, 0), (len - 1, len - 1)]
        .iter()
        .map(|&(r, c)| config[r][c].as_ref().unwrap().id())
        .product()
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
    fn test_example_input() {
        assert_eq!(calculate_part1("example.txt"), 20899048083289);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 30425930368573);
    }
}
