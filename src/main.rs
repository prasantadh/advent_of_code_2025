pub mod solution;
use paste::paste;

use clap::{Parser};

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

macro_rules! solution_factory {
    ( $($day:literal),+ $(,)? ) => {
        fn solution_factory(day: u8, part: u8) -> Box<dyn Solution> {
            match (day, part) {
                $(
                    ($day,1) => Box::new(paste! {crate::solution::[<day $day>]::Part1}),
                    ($day,2) => Box::new(paste! {crate::solution::[<day $day>]::Part2}),
                )+
                _ => panic!("Not yeet implemented"),
            }
        }
    };
}
solution_factory!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

fn main() {
    let args = Args::parse();
    let solver = solution_factory(args.day, args.part);
    let filename = format!("data/day{}/sample{}.txt", args.day, args.sample);
    println!("Answer: {}", solver.solve(&filename));
}
