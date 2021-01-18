use std::env;
use std::fs;
use std::mem;
use std::ops::Neg;
use std::path::Path;
use std::process;

enum Rotation {
    D090,
    D180,
    D270,
}

impl Rotation {
    fn from_degrees(degrees: isize) -> Self {
        match degrees {
            90 => Rotation::D090,
            180 => Rotation::D180,
            270 => Rotation::D270,
            _ => unreachable!(),
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(&mut self, rot: &Rotation) {
        *self = match self {
            Self::North => match rot {
                Rotation::D090 => Self::East,
                Rotation::D180 => Self::South,
                Rotation::D270 => Self::West,
            },
            Self::East => match rot {
                Rotation::D090 => Self::South,
                Rotation::D180 => Self::West,
                Rotation::D270 => Self::North,
            },
            Self::South => match rot {
                Rotation::D090 => Self::West,
                Rotation::D180 => Self::North,
                Rotation::D270 => Self::East,
            },
            Self::West => match rot {
                Rotation::D090 => Self::North,
                Rotation::D180 => Self::East,
                Rotation::D270 => Self::South,
            },
        }
    }

    fn rotate_anticlockwise(&mut self, rot: &Rotation) {
        *self = match self {
            Self::North => match rot {
                Rotation::D090 => Self::West,
                Rotation::D180 => Self::South,
                Rotation::D270 => Self::East,
            },
            Self::East => match rot {
                Rotation::D090 => Self::North,
                Rotation::D180 => Self::West,
                Rotation::D270 => Self::South,
            },
            Self::South => match rot {
                Rotation::D090 => Self::East,
                Rotation::D180 => Self::North,
                Rotation::D270 => Self::West,
            },
            Self::West => match rot {
                Rotation::D090 => Self::South,
                Rotation::D180 => Self::East,
                Rotation::D270 => Self::North,
            },
        }
    }
}

enum Move {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Right(Rotation),
    Left(Rotation),
    Forward(isize),
}

fn solve_part1(moves: &Vec<Move>) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::East;

    for mov in moves {
        match mov {
            Move::North(units) => y += units,
            Move::East(units) => x += units,
            Move::South(units) => y -= units,
            Move::West(units) => x -= units,
            Move::Right(rot) => direction.rotate_clockwise(&rot),
            Move::Left(rot) => direction.rotate_anticlockwise(&rot),
            Move::Forward(units) => match direction {
                Direction::North => y += units,
                Direction::East => x += units,
                Direction::South => y -= units,
                Direction::West => x -= units,
            },
        }
    }

    (x.abs() + y.abs()) as usize
}

fn solve_part2(moves: &Vec<Move>) -> usize {
    let (mut ship_x, mut ship_y): (isize, isize) = (0, 0);
    let (mut wp_x, mut wp_y): (isize, isize) = (10, 1);

    for mov in moves {
        match mov {
            Move::North(units) => wp_y += units,
            Move::East(units) => wp_x += units,
            Move::South(units) => wp_y -= units,
            Move::West(units) => wp_x -= units,
            Move::Right(rot) => rotate_coord_clockwise(&mut wp_x, &mut wp_y, rot),
            Move::Left(rot) => rotate_coord_anticlockwise(&mut wp_x, &mut wp_y, rot),
            Move::Forward(units) => {
                ship_x += wp_x * units;
                ship_y += wp_y * units;
            }
        }
    }

    (ship_x.abs() + ship_y.abs()) as usize
}

fn rotate_coord_clockwise<'a>(x: &'a mut isize, y: &'a mut isize, rot: &Rotation) {
    match rot {
        Rotation::D090 => {
            mem::swap(&mut *x, &mut *y);
            *y = y.neg();
        }
        Rotation::D180 => {
            *x = x.neg();
            *y = y.neg();
        }
        Rotation::D270 => {
            mem::swap(&mut *x, &mut *y);
            *x = x.neg();
        }
    }
}

fn rotate_coord_anticlockwise<'a>(x: &'a mut isize, y: &'a mut isize, rot: &Rotation) {
    match rot {
        Rotation::D090 => {
            mem::swap(&mut *x, &mut *y);
            *x = x.neg();
        }
        Rotation::D180 => {
            *x = x.neg();
            *y = y.neg();
        }
        Rotation::D270 => {
            mem::swap(&mut *x, &mut *y);
            *y = y.neg();
        }
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Move> {
    fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(|x| {
            let num = x[1..].parse::<isize>().unwrap();
            match x.chars().next().unwrap() {
                'N' => Move::North(num),
                'E' => Move::East(num),
                'S' => Move::South(num),
                'W' => Move::West(num),
                'R' => Move::Right(Rotation::from_degrees(num)),
                'L' => Move::Left(Rotation::from_degrees(num)),
                'F' => Move::Forward(num),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let moves = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&moves);
    let part2 = solve_part2(&moves);
    println!("Result (Part 1): {:?}", part1);
    println!("Result (Part 2): {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let moves = parse_input("example.txt");
        assert_eq!(solve_part1(&moves), 25);
        assert_eq!(solve_part2(&moves), 286);
    }

    #[test]
    fn test_puzzle_input() {
        let moves = parse_input("input.txt");
        assert_eq!(solve_part1(&moves), 1319);
        assert_eq!(solve_part2(&moves), 62434);
    }
}
