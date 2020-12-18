use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

struct Point(isize, isize, isize);

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

struct ConwayCubeSystem {
    matrix: Vec<Vec<Vec<State>>>,
    neighs: Vec<Point>,
}

impl ConwayCubeSystem {
    fn new(file_name: impl AsRef<Path>, max_cycles: usize) -> Self {
        let file = File::open(file_name).unwrap();
        let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

        let xylen = lines.len() + (max_cycles * 2);
        let zlen = 1 + (max_cycles * 2);
        let mut matrix = vec![vec![vec![State::Inactive; zlen]; xylen]; xylen];

        for (x, line) in lines.iter().enumerate() {
            for (y, state) in line.chars().enumerate() {
                let state = match state {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => unreachable!(),
                };
                matrix[x + max_cycles][y + max_cycles][max_cycles] = state;
            }
        }

        let neighs = (-1..=1)
            .flat_map(|x| {
                (-1..=1)
                    .flat_map(move |y| (-1..=1).clone().map(move |z| Point(x, y, z)))
                    .filter(|&Point(x, y, z)| !(x == 0 && y == 0 && z == 0))
            })
            .collect::<Vec<_>>();

        ConwayCubeSystem { matrix, neighs }
    }

    fn cyclen(&mut self, num_cycles: usize) {
        for _ in 0..num_cycles {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        let mut new_matrix = self.matrix.clone();

        // TODO: only scan for the points that might be active at certain cycle.
        for (x, xdim) in self.matrix.iter().enumerate() {
            for (y, ydim) in xdim.iter().enumerate() {
                for (z, point) in ydim.iter().enumerate() {
                    let num_active_neighs = self
                        .find_active_neighbors_count(&Point(x as isize, y as isize, z as isize));
                    match point {
                        State::Active => {
                            if !(num_active_neighs == 2 || num_active_neighs == 3) {
                                new_matrix[x][y][z] = State::Inactive;
                            }
                        }
                        State::Inactive => {
                            if num_active_neighs == 3 {
                                new_matrix[x][y][z] = State::Active;
                            }
                        }
                    }
                }
            }
        }

        self.matrix = new_matrix;
    }

    fn find_active_neighbors_count(&self, p: &Point) -> usize {
        self.neighs
            .iter()
            .filter(|neigh| self.is_active_neighbor(&p, &neigh))
            .count()
    }

    fn is_active_neighbor(&self, p: &Point, neigh: &Point) -> bool {
        let x = p.0 + neigh.0;
        let y = p.1 + neigh.1;
        let z = p.2 + neigh.2;
        let xylen = self.xylen() as isize;
        let zlen = self.zlen() as isize;
        if x < 0 || y < 0 || z < 0 || x >= xylen || y >= xylen || z >= zlen {
            return false;
        }
        self.matrix[x as usize][y as usize][z as usize] == State::Active
    }

    fn find_active_cubes_count(&self) -> usize {
        self.matrix
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter()))
            .filter(|&&x| x == State::Active)
            .count()
    }

    fn xylen(&self) -> usize {
        self.matrix.len()
    }

    fn zlen(&self) -> usize {
        self.matrix[0][0].len()
    }
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE CYCLES", env::args().next().unwrap());
        process::exit(1);
    }

    let file_name = env::args().nth(1).unwrap();
    let num_cycles = env::args().nth(2).unwrap().parse::<usize>().unwrap();
    let mut cube_system = ConwayCubeSystem::new(file_name, num_cycles);
    cube_system.cyclen(num_cycles);
    let part1 = cube_system.find_active_cubes_count();
    println!("Result (Part 1): {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let mut cube_system = ConwayCubeSystem::new("example.txt", 6);
        cube_system.cyclen(6);
        assert_eq!(cube_system.find_active_cubes_count(), 112);
    }

    #[test]
    fn test_puzzle_input() {
        let mut cube_system = ConwayCubeSystem::new("input.txt", 6);
        cube_system.cyclen(6);
        assert_eq!(cube_system.find_active_cubes_count(), 223);
    }
}
