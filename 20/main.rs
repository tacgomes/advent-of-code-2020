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

type Grid = Vec<Vec<Option<Tile>>>;
type Image = Vec<Vec<char>>;

fn rotate(matrix: &mut Image, n: usize) {
    assert_eq!(matrix.len(), matrix[0].len());
    let last = matrix.len() - 1;
    for _ in 0..n {
        for layer in 0..matrix.len() / 2 {
            for i in 1 + layer..matrix.len() - layer {
                let tmp = matrix[layer][i];
                matrix[layer][i] = matrix[last - i][layer];
                matrix[last - i][layer] = matrix[last - layer][last - i];
                matrix[last - layer][last - i] = matrix[i][last - layer];
                matrix[i][last - layer] = tmp;
            }
        }
    }
}

fn mirror(matrix: &mut Image) {
    matrix.reverse();
}

#[derive(Clone)]
struct Tile {
    tile_id: usize,
    data: Image,
    alignments: Vec<Vec<String>>,
    current_alignment: usize,
}

impl Tile {
    fn new(tile_id: usize, data: Image, borders: Vec<String>) -> Self {
        // TODO represent borders as integers that have a bit set at a
        // certain position, if the border has a '#' in the same
        // position. This would make border comparation fast.
        let mut alignments = vec![];

        let mut add_rotations = |border: Vec<String>| {
            alignments.push(border.clone());
            let mut border = border.to_vec();
            for _ in 0..3 {
                border = vec![
                    border[LEFT].chars().rev().clone().collect(),
                    border[TOP].clone(),
                    border[RIGHT].chars().rev().clone().collect(),
                    border[BOTTOM].clone(),
                ];
                alignments.push(border.clone());
            }
        };

        let mirrored = vec![
            borders[BOTTOM].clone(),
            borders[RIGHT].chars().rev().clone().collect(),
            borders[TOP].clone(),
            borders[LEFT].chars().rev().clone().collect(),
        ];

        add_rotations(mirrored);
        add_rotations(borders);

        Tile {
            tile_id,
            data,
            alignments,
            current_alignment: 0,
        }
    }

    fn id(&self) -> usize {
        self.tile_id
    }

    fn next_border_alignment(&mut self) {
        self.current_alignment += 1;
        self.current_alignment %= self.alignments.len();
    }

    fn apply_border_alignment(&mut self) {
        if self.current_alignment < 4 {
            mirror(&mut self.data);
        }
        match self.current_alignment {
            0 | 4 => rotate(&mut self.data, 0),
            1 | 5 => rotate(&mut self.data, 1),
            2 | 6 => rotate(&mut self.data, 2),
            3 | 7 => rotate(&mut self.data, 3),
            _ => unreachable!(),
        }
    }

    fn top_border(&self) -> &str {
        &self.alignments[self.current_alignment][TOP]
    }

    fn right_border(&self) -> &str {
        &self.alignments[self.current_alignment][RIGHT]
    }

    fn bottom_border(&self) -> &str {
        &self.alignments[self.current_alignment][BOTTOM]
    }

    fn left_border(&self) -> &str {
        &self.alignments[self.current_alignment][LEFT]
    }

    fn data(&self) -> &Image {
        &self.data
    }
}

#[allow(clippy::ptr_arg)]
fn validate_alignment(grid: &Grid) -> bool {
    let aligned_v = |x: &Option<Tile>, y: &Option<Tile>| {
        x.as_ref().unwrap().right_border() == y.as_ref().unwrap().left_border()
    };

    let aligned_h = |x: &Option<Tile>, y: &Option<Tile>| {
        x.as_ref().unwrap().bottom_border() == y.as_ref().unwrap().top_border()
    };

    grid.iter().all(|r| {
        r.windows(2)
            .filter(|w| w[1].is_some())
            .all(|w| aligned_v(&w[0], &w[1]))
    }) && grid.windows(2).all(|w| {
        w[0].iter()
            .zip(&w[1])
            .filter(|(_, y)| y.is_some())
            .all(|(x, y)| aligned_h(x, y))
    })
}

fn find_valid_alignment(
    mut tiles: &mut VecDeque<Tile>,
    row: usize,
    col: usize,
    mut grid: &mut Grid,
) -> bool {
    if !validate_alignment(&grid) {
        return false;
    }

    if row == grid.len() {
        return true;
    }

    let (next_row, next_col) = match col + 1 == grid.len() {
        true => (row + 1, 0),
        false => (row, col + 1),
    };

    for _ in 0..tiles.len() {
        grid[row][col] = Some(tiles.pop_front().unwrap());
        if let true = (0..8).any(|_| {
            grid[row][col].as_mut().unwrap().next_border_alignment();
            find_valid_alignment(&mut tiles, next_row, next_col, &mut grid)
        }) {
            return true;
        }
        tiles.push_back(mem::replace(&mut grid[row][col], None).unwrap());
    }

    grid[row][col] = None;
    false
}

#[allow(clippy::ptr_arg)]
fn assemble_image(grid: &Grid) -> Image {
    let tile_len = grid[0][0].as_ref().unwrap().data().len();
    grid.iter()
        .flat_map(|row| {
            (1..tile_len - 1).map(move |i| {
                row.iter()
                    .flat_map(|t| {
                        t.as_ref().unwrap().data()[i]
                            .iter()
                            .skip(1)
                            .take(t.as_ref().unwrap().data()[i].len() - 2)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<Vec<_>>>()
}

fn sea_monsters_count_util(image: &mut Image) -> usize {
    let mut num_monsters = 0;
    for (r, _) in image.iter().enumerate().skip(1).take(image.len() - 2) {
        let mut c = 0;
        while c < image[0].len() - 19 {
            if image[r][c] == '#'
                && image[r - 1][c + 18] == '#'
                && image[r][c + 5] == '#'
                && image[r][c + 6] == '#'
                && image[r][c + 11] == '#'
                && image[r][c + 12] == '#'
                && image[r][c + 17] == '#'
                && image[r][c + 18] == '#'
                && image[r][c + 19] == '#'
                && image[r + 1][c + 1] == '#'
                && image[r + 1][c + 4] == '#'
                && image[r + 1][c + 7] == '#'
                && image[r + 1][c + 10] == '#'
                && image[r + 1][c + 13] == '#'
                && image[r + 1][c + 16] == '#'
            {
                num_monsters += 1;
                c += 19;
            }
            c += 1;
        }
    }
    num_monsters
}

fn sea_monsters_count(image: &mut Image) -> usize {
    for _ in 0..4 {
        match sea_monsters_count_util(image) {
            0 => rotate(image, 1),
            count => return count,
        }
    }

    mirror(image);

    for _ in 0..4 {
        match sea_monsters_count_util(image) {
            0 => rotate(image, 1),
            count => return count,
        }
    }

    0
}

fn read_tiles(file_name: impl AsRef<Path>) -> VecDeque<Tile> {
    let content = fs::read_to_string(file_name).unwrap();
    let blocks = content.trim().split("\n\n").collect::<Vec<_>>();

    let mut tiles = VecDeque::new();

    for tile in blocks {
        let mut lines = tile.split('\n');
        let line = &lines.next().unwrap();
        let tile_id = line[5..line.len() - 1].parse::<usize>().unwrap();

        let data = lines
            .clone()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let borders = vec![
            lines.clone().next().unwrap().to_string(),
            lines
                .clone()
                .map(|l| l.chars().last().unwrap())
                .collect::<String>(),
            lines.clone().last().unwrap().to_string(),
            lines
                .clone()
                .map(|l| l.chars().next().unwrap())
                .collect::<String>(),
        ];

        tiles.push_back(Tile::new(tile_id, data, borders));
    }

    tiles
}

fn solve(file_name: impl AsRef<Path>) -> (usize, usize) {
    let mut tiles = read_tiles(file_name);

    let len = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec![None; len]; len];

    assert!(find_valid_alignment(&mut tiles, 0, 0, &mut grid));

    grid.iter_mut()
        .flat_map(|x| x.iter_mut())
        .for_each(|t| t.as_mut().unwrap().apply_border_alignment());

    let mut image = assemble_image(&grid);

    let corners_product = [(0, 0), (0, len - 1), (len - 1, 0), (len - 1, len - 1)]
        .iter()
        .map(|&(r, c)| grid[r][c].as_ref().unwrap().id())
        .product();

    let roughness = image
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&&c| c == '#')
        .count()
        - sea_monsters_count(&mut image) * 15;

    (corners_product, roughness)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (corners_product, roughness) = solve(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", corners_product);
    println!("Result (Part 2): {}", roughness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(solve("example.txt"), (20899048083289, 273));
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve("input.txt"), (30425930368573, 2453));
    }
}
