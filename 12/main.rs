use std::env;
use std::fs;
use std::path::Path;
use std::process;

use num::complex::Complex;

enum Move {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

fn re(n: isize) -> Complex<isize> {
    Complex { re: n, im: 0 }
}

fn im(n: isize) -> Complex<isize> {
    Complex { re: 0, im: n }
}

fn nrots(n: isize) -> u32 {
    n as u32 / 90
}

fn solve_part1(moves: &Vec<Move>) -> isize {
    let mut coord = re(0);
    let mut dir = re(1);

    for mov in moves {
        match mov {
            Move::North(n) => coord += im(*n),
            Move::East(n) => coord += re(*n),
            Move::South(n) => coord += -im(*n),
            Move::West(n) => coord += -re(*n),
            Move::Left(r) => dir *= im(1).powu(nrots(*r)),
            Move::Right(r) => dir *= im(-1).powu(nrots(*r)),
            Move::Forward(n) => coord += dir * n,
        }
    }

    coord.l1_norm()
}

fn solve_part2(moves: &Vec<Move>) -> isize {
    let mut ship = re(0);
    let mut waypoint = Complex { re: 10, im: 1 };

    for mov in moves {
        match mov {
            Move::North(n) => waypoint += im(*n),
            Move::East(n) => waypoint += re(*n),
            Move::South(n) => waypoint += -im(*n),
            Move::West(n) => waypoint += -re(*n),
            Move::Left(r) => waypoint *= im(1).powu(nrots(*r)),
            Move::Right(r) => waypoint *= im(-1).powu(nrots(*r)),
            Move::Forward(n) => ship += waypoint * n,
        }
    }

    ship.l1_norm()
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
                'R' => Move::Right(num),
                'L' => Move::Left(num),
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
