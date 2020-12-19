use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

struct Point(isize, isize, isize, isize);

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

struct ConwayCubeSystem {
    matrix: Vec<Vec<Vec<Vec<State>>>>,
    neighs: Vec<Point>,
}

impl ConwayCubeSystem {
    fn new(file_name: impl AsRef<Path>, max_cycles: usize) -> Self {
        let file = File::open(file_name).unwrap();
        let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

        let xylen = lines.len() + (max_cycles * 2);
        let zwlen = 1 + (max_cycles * 2);
        let mut matrix = vec![vec![vec![vec![State::Inactive; zwlen]; zwlen]; xylen]; xylen];

        for (x, line) in lines.iter().enumerate() {
            for (y, state) in line.chars().enumerate() {
                let state = match state {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => unreachable!(),
                };
                matrix[x + max_cycles][y + max_cycles][max_cycles][max_cycles] = state;
            }
        }

        // Calculate cartesian product
        let neighs = (-1..=1)
            .flat_map(|x| {
                (-1..=1)
                    .flat_map(move |y| {
                        (-1..=1)
                            .clone()
                            .flat_map(move |z| (-1..=1).clone().map(move |w| Point(x, y, z, w)))
                    })
                    .filter(|&Point(x, y, z, w)| !(x == 0 && y == 0 && z == 0 && w == 0))
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
                for (z, zdim) in ydim.iter().enumerate() {
                    for (w, point) in zdim.iter().enumerate() {
                        let num_active_neighs = self.find_active_neighbors_count(&Point(
                            x as isize, y as isize, z as isize, w as isize,
                        ));
                        match point {
                            State::Active => {
                                if !(num_active_neighs == 2 || num_active_neighs == 3) {
                                    new_matrix[x][y][z][w] = State::Inactive;
                                }
                            }
                            State::Inactive => {
                                if num_active_neighs == 3 {
                                    new_matrix[x][y][z][w] = State::Active;
                                }
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

    #[allow(clippy::many_single_char_names)]
    fn is_active_neighbor(&self, p: &Point, neigh: &Point) -> bool {
        let x = p.0 + neigh.0;
        let y = p.1 + neigh.1;
        let z = p.2 + neigh.2;
        let w = p.3 + neigh.3;
        let xylen = self.xylen() as isize;
        let zwlen = self.zwlen() as isize;
        if x < 0 || y < 0 || z < 0 || w < 0 {
            return false;
        }
        if x >= xylen || y >= xylen || z >= zwlen || w >= zwlen {
            return false;
        }
        self.matrix[x as usize][y as usize][z as usize][w as usize] == State::Active
    }

    fn find_active_cubes_count(&self) -> usize {
        self.matrix
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().flat_map(|z| z.iter())))
            .filter(|&&x| x == State::Active)
            .count()
    }

    fn xylen(&self) -> usize {
        self.matrix.len()
    }

    fn zwlen(&self) -> usize {
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
    let part2 = cube_system.find_active_cubes_count();
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let mut cube_system = ConwayCubeSystem::new("example.txt", 6);
        cube_system.cyclen(6);
        assert_eq!(cube_system.find_active_cubes_count(), 848);
    }

    #[test]
    fn test_puzzle_input() {
        let mut cube_system = ConwayCubeSystem::new("input.txt", 6);
        cube_system.cyclen(6);
        assert_eq!(cube_system.find_active_cubes_count(), 1884);
    }
}
