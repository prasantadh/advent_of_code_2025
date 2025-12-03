# Advent of Cyber 2025

This code repo is my solution to the
[Advent of Code 2025](https://adventofcode.com/2025/about)

## Pre-requisite

The solution is done in [rust](https://rust-lang.org/).
You can install rust from [rustup](https://rustup.rs/)

## Usage

```bash
$ cargo run -- --help
Usage: aoc [OPTIONS]

Options:
  -d, --day <DAY>        [default: 1]
  -p, --part <PART>      [default: 1]
  -s, --sample <SAMPLE>  [default: 0]
  -h, --help             Print help
  -V, --version          Print version

$ cargo run -- --day=1 --part=2 --sample=0 
```

## Known Issues

- `data` folder is missing. This is to comply with the official Advent of Code
policy. Copying the input is not allowed. The program expects the input
in the form `/data/day{1-12}/sample{X}.txt`. `X` maps to the value provided in
`--sample` option while running the program.

