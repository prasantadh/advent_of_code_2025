pub mod solution;

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,

    #[arg(short, long, default_value_t = 0)]
    sample: u8,
}

pub trait Solution {
    fn solve(&self, filename: &str) -> i64;
}

fn solution_factory(day: u8, part: u8) -> Box<dyn Solution> {
    match (day, part) {
        (1, 1) => Box::new(crate::solution::day1::Part1),
        (1, 2) => Box::new(crate::solution::day1::Part2),
        (2, 1) => Box::new(crate::solution::day2::Part1),
        (2, 2) => Box::new(crate::solution::day2::Part2),
        (3, 1) => Box::new(crate::solution::day3::Part1),
        (3, 2) => Box::new(crate::solution::day3::Part2),
        (4, 1) => Box::new(crate::solution::day4::Part1),
        (4, 2) => Box::new(crate::solution::day4::Part2),
        (5, 1) => Box::new(crate::solution::day5::Part1),
        (5, 2) => Box::new(crate::solution::day5::Part2),
        _ => panic!("Not yet implemented!"),
    }
}

fn main() {
    let args = Args::parse();
    let solver = solution_factory(args.day, args.part);
    let filename = format!("data/day{}/sample{}.txt", args.day, args.sample);
    println!("Answer: {}", solver.solve(&*filename));
}
