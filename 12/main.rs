use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
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
    fn rotate_clockwise(&mut self, rotation: &Rotation) {
        *self = match self {
            Self::North => match rotation {
                Rotation::D090 => Self::East,
                Rotation::D180 => Self::South,
                Rotation::D270 => Self::West,
            },
            Self::East => match rotation {
                Rotation::D090 => Self::South,
                Rotation::D180 => Self::West,
                Rotation::D270 => Self::North,
            },
            Self::South => match rotation {
                Rotation::D090 => Self::West,
                Rotation::D180 => Self::North,
                Rotation::D270 => Self::East,
            },
            Self::West => match rotation {
                Rotation::D090 => Self::North,
                Rotation::D180 => Self::East,
                Rotation::D270 => Self::South,
            },
        }
    }

    fn rotate_anticlockwise(&mut self, rotation: &Rotation) {
        *self = match self {
            Self::North => match rotation {
                Rotation::D090 => Self::West,
                Rotation::D180 => Self::South,
                Rotation::D270 => Self::East,
            },
            Self::East => match rotation {
                Rotation::D090 => Self::North,
                Rotation::D180 => Self::West,
                Rotation::D270 => Self::South,
            },
            Self::South => match rotation {
                Rotation::D090 => Self::East,
                Rotation::D180 => Self::North,
                Rotation::D270 => Self::West,
            },
            Self::West => match rotation {
                Rotation::D090 => Self::South,
                Rotation::D180 => Self::East,
                Rotation::D270 => Self::North,
            },
        }
    }
}

enum Movement {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Right(Rotation),
    Left(Rotation),
    Forward(isize),
}

struct ShipNavigation {
    movements: Vec<Movement>,
}

impl ShipNavigation {
    fn from_file(file_name: impl AsRef<Path>) -> Self {
        let file = File::open(file_name).unwrap();
        let lines = BufReader::new(file).lines();

        let mut movements = vec![];

        for line in lines {
            let line = line.unwrap();
            let ch = line.chars().next().unwrap();
            let num = line[1..].parse::<isize>().unwrap();
            let mov = match ch as char {
                'N' => Movement::North(num),
                'E' => Movement::East(num),
                'S' => Movement::South(num),
                'W' => Movement::West(num),
                'R' => Movement::Right(Rotation::from_degrees(num)),
                'L' => Movement::Left(Rotation::from_degrees(num)),
                'F' => Movement::Forward(num),
                _ => unreachable!(),
            };
            movements.push(mov);
        }

        ShipNavigation { movements }
    }

    fn navigate_part1(&self) -> usize {
        let (mut x, mut y): (isize, isize) = (0, 0);
        let mut direction = Direction::East;

        for mov in &self.movements {
            match mov {
                Movement::North(units) => y += units,
                Movement::East(units) => x += units,
                Movement::South(units) => y -= units,
                Movement::West(units) => x -= units,
                Movement::Right(rotation) => direction.rotate_clockwise(&rotation),
                Movement::Left(rotation) => direction.rotate_anticlockwise(&rotation),
                Movement::Forward(units) => match direction {
                    Direction::North => y += units,
                    Direction::East => x += units,
                    Direction::South => y -= units,
                    Direction::West => x -= units,
                },
            }
        }

        (x.abs() + y.abs()) as usize
    }

    fn navigate_part2(&self) -> usize {
        let (mut ship_x, mut ship_y): (isize, isize) = (0, 0);
        let (mut wp_x, mut wp_y): (isize, isize) = (10, 1);

        for mov in &self.movements {
            match mov {
                Movement::North(units) => wp_y += units,
                Movement::East(units) => wp_x += units,
                Movement::South(units) => wp_y -= units,
                Movement::West(units) => wp_x -= units,
                Movement::Right(rotation) => match rotation {
                    Rotation::D090 => {
                        mem::swap(&mut wp_x, &mut wp_y);
                        wp_y = wp_y.neg();
                    }
                    Rotation::D180 => {
                        wp_x *= -1;
                        wp_y *= -1;
                    }
                    Rotation::D270 => {
                        mem::swap(&mut wp_x, &mut wp_y);
                        wp_x = wp_x.neg();
                    }
                },
                Movement::Left(rotation) => match rotation {
                    Rotation::D090 => {
                        mem::swap(&mut wp_x, &mut wp_y);
                        wp_x = wp_x.neg();
                    }
                    Rotation::D180 => {
                        wp_x = wp_x.neg();
                        wp_y = wp_y.neg();
                    }
                    Rotation::D270 => {
                        mem::swap(&mut wp_x, &mut wp_y);
                        wp_y = wp_y.neg();
                    }
                },
                Movement::Forward(units) => {
                    ship_x += wp_x * units;
                    ship_y += wp_y * units;
                }
            }
        }

        (ship_x.abs() + ship_y.abs()) as usize
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let ship_navigation = ShipNavigation::from_file(env::args().nth(1).unwrap());
    let distance_part1 = ship_navigation.navigate_part1();
    let distance_part2 = ship_navigation.navigate_part2();
    println!("Result (Part 1): {:?}", distance_part1);
    println!("Result (Part 2): {:?}", distance_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let ship_navigation = ShipNavigation::from_file("example.txt");
        assert_eq!(ship_navigation.navigate_part1(), 25);
        assert_eq!(ship_navigation.navigate_part2(), 286);
    }

    #[test]
    fn test_puzzle_input() {
        let ship_navigation = ShipNavigation::from_file("input.txt");
        assert_eq!(ship_navigation.navigate_part1(), 1319);
        assert_eq!(ship_navigation.navigate_part2(), 62434);
    }
}
