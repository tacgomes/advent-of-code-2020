use std::env;
use std::fs;
use std::process;

use itertools::iproduct;

struct Point(isize, isize, isize, isize);

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

fn cartesian_product(hypercube: bool) -> Vec<Point> {
    let wrange = if hypercube { -1..2 } else { 0..1 };
    iproduct!(-1..2, -1..2, -1..2, wrange)
        .filter(|&(x, y, z, w)| !(x == 0 && y == 0 && z == 0 && w == 0))
        .map(|(x, y, z, w)| Point(x, y, z, w))
        .collect()
}

struct ConwayCubeSystem {
    cubes: Vec<Vec<Vec<Vec<State>>>>,
    moves: Vec<Point>,
    num_cycles: usize,
}

impl ConwayCubeSystem {
    fn new(s: &str, num_cycles: usize, hypercube: bool) -> Self {
        let xylen = s.lines().count() + (num_cycles * 2);
        let zwlen = 1 + (num_cycles * 2);
        let mut cubes = vec![vec![vec![vec![State::Inactive; zwlen]; zwlen]; xylen]; xylen];

        for (x, line) in s.lines().enumerate() {
            for (y, state) in line.chars().enumerate() {
                let state = match state {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => unreachable!(),
                };
                cubes[x + num_cycles][y + num_cycles][num_cycles][num_cycles] = state;
            }
        }

        let moves = cartesian_product(hypercube);

        ConwayCubeSystem {
            cubes,
            moves,
            num_cycles,
        }
    }

    fn iterate(&mut self) {
        let mut new_cubes = self.cubes.clone();

        // NB: we could optmise here by scanning only for the points
        // that might be active at certain cycle.
        for (x, xdim) in self.cubes.iter().enumerate() {
            for (y, ydim) in xdim.iter().enumerate() {
                for (z, zdim) in ydim.iter().enumerate() {
                    for (w, point) in zdim.iter().enumerate() {
                        let num_active_neighbors = self.count_active_neighbors(&Point(
                            x as isize, y as isize, z as isize, w as isize,
                        ));
                        match point {
                            State::Active => {
                                if !(num_active_neighbors == 2 || num_active_neighbors == 3) {
                                    new_cubes[x][y][z][w] = State::Inactive;
                                }
                            }
                            State::Inactive => {
                                if num_active_neighbors == 3 {
                                    new_cubes[x][y][z][w] = State::Active;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.cubes = new_cubes;
    }

    fn count_active_neighbors(&self, p: &Point) -> usize {
        self.moves
            .iter()
            .filter(|neigh| self.is_active(&p, &neigh))
            .count()
    }

    #[allow(clippy::many_single_char_names)]
    fn is_active(&self, p: &Point, mov: &Point) -> bool {
        let x = p.0 + mov.0;
        let y = p.1 + mov.1;
        let z = p.2 + mov.2;
        let w = p.3 + mov.3;
        if x < 0 || y < 0 || z < 0 || w < 0 {
            return false;
        }

        let xylen = self.xylen() as isize;
        let zwlen = self.zwlen() as isize;
        if x >= xylen || y >= xylen || z >= zwlen || w >= zwlen {
            return false;
        }

        self.cubes[x as usize][y as usize][z as usize][w as usize] == State::Active
    }

    fn count_active_cubes(&mut self) -> usize {
        for _ in 0..self.num_cycles {
            self.iterate();
        }
        self.cubes
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().flat_map(|z| z.iter())))
            .filter(|&&x| x == State::Active)
            .count()
    }

    fn xylen(&self) -> usize {
        self.cubes.len()
    }

    fn zwlen(&self) -> usize {
        self.cubes[0][0].len()
    }
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE CYCLES", env::args().next().unwrap());
        process::exit(1);
    }

    let input = fs::read_to_string(&env::args().nth(1).unwrap()).unwrap();
    let num_cycles = env::args().nth(2).unwrap().parse::<usize>().unwrap();

    let mut cube_system = ConwayCubeSystem::new(&input, num_cycles, false);
    let part1 = cube_system.count_active_cubes();

    let mut cube_system = ConwayCubeSystem::new(&input, num_cycles, true);
    let part2 = cube_system.count_active_cubes();

    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        let mut cube_system = ConwayCubeSystem::new(&input, 6, false);
        assert_eq!(cube_system.count_active_cubes(), 112);
    }

    #[test]
    fn test_example_input_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        let mut cube_system = ConwayCubeSystem::new(&input, 6, true);
        assert_eq!(cube_system.count_active_cubes(), 848);
    }

    #[test]
    fn test_puzzle_input_part1() {
        let input = fs::read_to_string("input.txt").unwrap();
        let mut cube_system = ConwayCubeSystem::new(&input, 6, false);
        assert_eq!(cube_system.count_active_cubes(), 223);
    }

    #[test]
    fn test_puzzle_input_part2() {
        let input = fs::read_to_string("input.txt").unwrap();
        let mut cube_system = ConwayCubeSystem::new(&input, 6, true);
        assert_eq!(cube_system.count_active_cubes(), 1884);
    }
}
