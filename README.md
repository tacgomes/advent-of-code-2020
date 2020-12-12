# Advent of Code 2020

![Rust workflow](https://github.com/tacgomes/AoC-2020/workflows/Rust/badge.svg)

My solutions in Rust to the [Advent of Code], 2020 edition.

I deliberately choose not to use any external Cargo package to practice
exercising the core features of the language. The exception to this rule was on
_Puzzle 07_ where not using an external package such as Regex would make
parsing the input a very tedious task. For the sake of simplicity, I skipped
all error handling and therefore all the programs assume valid input.

## Building and Running

All the programs are organised in a Cargo workspace. Running the following
instructions will build all the programs and, as an example, run the program to
solve the _Part 1_ of the _Puzzle 01_:

```sh
cargo build
cargo run --bin 01-part1 01/input.txt
```

Most programs do not depend on any external package and therefore can be build
directly with `rustc`:

```sh
cd 01
rustc part1.rs
./part1 input.txt
```

[Advent of Code]: https://adventofcode.com/2020/about
