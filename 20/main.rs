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

const SEA_MONSTER: &str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #";

type Grid = [Vec<Option<Tile>>];
type Image = Vec<Vec<char>>;

fn mirror(matrix: &mut Image) {
    matrix.reverse();
}

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

fn hash_border(border: &str) -> u16 {
    border
        .chars()
        .rev()
        .enumerate()
        .filter(|&(_, x)| x == '#')
        .fold(0, |acc, (i, _)| acc | 1 << i)
}

fn add_rotations(mut border: Vec<String>) -> Vec<Vec<u16>> {
    let mut alignments = vec![];
    for _ in 0..4 {
        alignments.push(border.iter().map(|x| hash_border(x)).collect());
        border = vec![
            border[LEFT].chars().rev().collect(),
            border[TOP].clone(),
            border[RIGHT].chars().rev().collect(),
            border[BOTTOM].clone(),
        ];
    }
    alignments
}

#[derive(Clone)]
struct Tile {
    id: usize,
    image: Image,
    alignments: Vec<Vec<u16>>,
    current_alignment: usize,
}

impl Tile {
    fn new(id: usize, image: Image, borders: Vec<String>) -> Self {
        let mirrored = vec![
            borders[BOTTOM].clone(),
            borders[RIGHT].chars().rev().collect(),
            borders[TOP].clone(),
            borders[LEFT].chars().rev().collect(),
        ];

        let alignments = add_rotations(mirrored)
            .into_iter()
            .chain(add_rotations(borders).into_iter())
            .collect();

        Tile {
            id,
            image,
            alignments,
            current_alignment: 0,
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn next_border_alignment(&mut self) {
        self.current_alignment += 1;
        self.current_alignment %= self.alignments.len();
    }

    fn apply_border_alignment(&mut self) {
        if self.current_alignment < 4 {
            mirror(&mut self.image);
        }
        match self.current_alignment {
            0 | 4 => rotate(&mut self.image, 0),
            1 | 5 => rotate(&mut self.image, 1),
            2 | 6 => rotate(&mut self.image, 2),
            3 | 7 => rotate(&mut self.image, 3),
            _ => unreachable!(),
        }
    }

    fn top_border(&self) -> u16 {
        self.alignments[self.current_alignment][TOP]
    }

    fn right_border(&self) -> u16 {
        self.alignments[self.current_alignment][RIGHT]
    }

    fn bottom_border(&self) -> u16 {
        self.alignments[self.current_alignment][BOTTOM]
    }

    fn left_border(&self) -> u16 {
        self.alignments[self.current_alignment][LEFT]
    }

    fn image(&self) -> &Image {
        &self.image
    }
}

fn vertically_aligned(grid: &Grid) -> bool {
    let aligned = |x: &Option<Tile>, y: &Option<Tile>| {
        x.as_ref().unwrap().right_border() == y.as_ref().unwrap().left_border()
    };
    grid.iter().all(|r| {
        r.windows(2)
            .filter(|w| w[1].is_some())
            .all(|w| aligned(&w[0], &w[1]))
    })
}

fn horizontally_aligned(grid: &Grid) -> bool {
    let aligned = |x: &Option<Tile>, y: &Option<Tile>| {
        x.as_ref().unwrap().bottom_border() == y.as_ref().unwrap().top_border()
    };
    grid.windows(2).all(|w| {
        w[0].iter()
            .zip(&w[1])
            .filter(|(_, y)| y.is_some())
            .all(|(x, y)| aligned(x, y))
    })
}

fn aligned(grid: &Grid) -> bool {
    vertically_aligned(grid) && horizontally_aligned(grid)
}

fn find_valid_alignment(
    mut tiles: &mut VecDeque<Tile>,
    row: usize,
    col: usize,
    mut grid: &mut Grid,
) -> bool {
    if !aligned(&grid) {
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

fn assemble_image(grid: &Grid) -> Image {
    let tile_len = grid[0][0].as_ref().unwrap().image().len();
    grid.iter()
        .flat_map(|row| {
            (1..tile_len - 1).map(move |i| {
                row.iter()
                    .flat_map(|t| {
                        t.as_ref().unwrap().image()[i]
                            .iter()
                            .skip(1)
                            .take(t.as_ref().unwrap().image()[i].len() - 2)
                    })
                    .cloned()
                    .collect()
            })
        })
        .collect()
}

fn match_pattern(image: &mut Image, pattern: &[(usize, usize)]) -> usize {
    let max_row = pattern.iter().map(|(r, _)| r).max().unwrap();
    let max_col = pattern.iter().map(|(_, c)| c).max().unwrap();
    let mut matches = 0;

    for (r, _) in image.iter().enumerate().take(image.len() - max_row) {
        let mut c = 0;
        while c < image[0].len() - max_col {
            if pattern.iter().all(|(dr, dc)| image[r + dr][c + dc] == '#') {
                matches += 1;
                c += max_col;
            } else {
                c += 1;
            }
        }
    }

    matches
}

fn compile_pattern(pattern: &str) -> Vec<(usize, usize)> {
    pattern
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, s)| s == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect()
}

fn count_sea_monsters(image: &mut Image) -> usize {
    let pattern = compile_pattern(SEA_MONSTER);

    for _ in 0..4 {
        match match_pattern(image, &pattern) {
            0 => rotate(image, 1),
            count => return count,
        }
    }

    mirror(image);

    for _ in 0..4 {
        match match_pattern(image, &pattern) {
            0 => rotate(image, 1),
            count => return count,
        }
    }

    0
}

fn parse_tile(tile: &str) -> Tile {
    let mut lines = tile.split('\n');
    let line = lines.next().unwrap();
    let id = line[5..line.len() - 1].parse().unwrap();

    let image = lines.clone().map(|x| x.chars().collect()).collect();

    let borders = vec![
        lines.clone().next().unwrap().to_string(),
        lines.clone().map(|x| x.chars().last().unwrap()).collect(),
        lines.clone().last().unwrap().to_string(),
        lines.clone().map(|x| x.chars().next().unwrap()).collect(),
    ];

    Tile::new(id, image, borders)
}

fn parse_input(file_name: impl AsRef<Path>) -> VecDeque<Tile> {
    fs::read_to_string(file_name)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| parse_tile(x))
        .collect()
}

fn solve(mut tiles: VecDeque<Tile>) -> (usize, usize) {
    let len = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec![None; len]; len];

    assert!(find_valid_alignment(&mut tiles, 0, 0, &mut grid));

    grid.iter_mut()
        .flat_map(|x| x.iter_mut())
        .for_each(|t| t.as_mut().unwrap().apply_border_alignment());

    let corners_product = [(0, 0), (0, len - 1), (len - 1, 0), (len - 1, len - 1)]
        .iter()
        .map(|&(r, c)| grid[r][c].as_ref().unwrap().id())
        .product();

    let mut image = assemble_image(&grid);
    let roughness = image
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&&c| c == '#')
        .count()
        - count_sea_monsters(&mut image) * 15;

    (corners_product, roughness)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let tiles = parse_input(env::args().nth(1).unwrap());
    let (corners_product, roughness) = solve(tiles);
    println!("Result (Part 1): {}", corners_product);
    println!("Result (Part 2): {}", roughness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let tiles = parse_input("example.txt");
        assert_eq!(solve(tiles), (20899048083289, 273));
    }

    #[test]
    fn test_puzzle_input() {
        let tiles = parse_input("input.txt");
        assert_eq!(solve(tiles), (30425930368573, 2453));
    }
}
