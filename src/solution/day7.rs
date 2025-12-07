use crate::Solution;
use core::fmt::Display;
use std::fs;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let mut manifold = read_input(filename);
        let mut answer = 0;
        for i in 1..manifold.len() {
            for j in 0..manifold[i].len() {
                match manifold[i - 1][j] {
                    Manifold::Start => manifold[i][j] = Manifold::Beam,
                    Manifold::Beam => {
                        if manifold[i][j] == Manifold::Splitter {
                            manifold[i][j - 1] = Manifold::Beam;
                            manifold[i][j + 1] = Manifold::Beam;
                            answer += 1;
                        } else {
                            manifold[i][j] = Manifold::Beam;
                        }
                    }
                    _ => (),
                }
            }
        }

        for line in manifold {
            for item in line {
                print!("{}", item);
            }
            println!();
        }

        answer as i64
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let mut manifold = read_input(filename);
        let mut answer = 0;
        answer as i64
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Manifold {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::Splitter => write!(f, "^"),
            Self::Beam => write!(f, "|"),
        }
    }
}

fn read_input(filename: &str) -> Vec<Vec<Manifold>> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input file", filename))
        .trim()
        .split("\n")
        .map(|v| {
            v.chars()
                .map(|c| match c {
                    'S' => Manifold::Start,
                    '^' => Manifold::Splitter,
                    _ => Manifold::Empty,
                })
                .collect::<Vec<Manifold>>()
        })
        .collect()
}
