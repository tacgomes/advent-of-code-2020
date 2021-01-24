# Advent of Code 2020

![Rust workflow](https://github.com/tacgomes/AoC-2020/workflows/Rust/badge.svg)

My solutions in Rust to the [Advent of Code], 2020 edition.

## Building and Running

All the programs are organised in a Cargo workspace. Running the following
instructions will build all the programs and, as an example, run the program to
solve the _Part 1_ of the _Puzzle 01_:

```sh
cargo build
cargo run --bin 01-part1 01/input.txt
```

Most programs do not depend on any external package and can therefore be
build directly with `rustc`:

```sh
cd 01
rustc main.rs
./main input.txt
```

[Advent of Code]: https://adventofcode.com/2020/about
