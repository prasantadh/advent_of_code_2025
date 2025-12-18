use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

#[derive(Clone, Copy, Debug)]
struct Present([[bool; 3]; 3]);

impl Present {
    fn size(&self) -> u64 {
        let mut answer = 0;
        for row in self.0 {
            for col in row {
                answer += col as u64
            }
        }
        answer
    }
}

#[derive(Debug)]
struct Tree {
    width: u64,
    height: u64,
    count: Vec<u64>,
}

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let input = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("should have been able to read {} as input", filename));
        let mut input = input.trim().split("\n");

        // parse the presents
        let mut presents = [Present([[false; 3]; 3]); 6];
        for present in presents.iter_mut() {
            input.next(); // ignore the line with numbers such as 0:
            for row in present.0.iter_mut() {
                for (k, c) in input.next().unwrap().chars().enumerate() {
                    row[k] = c == '#';
                }
            }
            input.next(); // ignore the empty line at the end of each present
        }

        // parse the trees
        let mut trees: Vec<Tree> = vec![];
        for line in input {
            let mut words = line.split_whitespace();
            let mut dim = words.next().unwrap().split('x');
            let width = dim.next().unwrap().parse::<u64>().unwrap();
            let height = dim
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<u64>()
                .unwrap();
            let mut count: Vec<u64> = vec![];
            for value in words {
                count.push(value.parse::<u64>().unwrap());
            }
            trees.push(Tree {
                width,
                height,
                count,
            });
        }

        // check if there is enough space for all the presents.
        // If yes, just assume they fit
        let mut answer = 0;
        for tree in trees {
            let mut needed = 0;
            for (i, value) in tree.count.iter().enumerate() {
                needed += *value * presents[i].size();
            }
            if tree.height * tree.width > needed {
                answer += 1;
            }
        }
        answer
    }
}

impl Solution for Part2 {
    fn solve(&self, _filename: &str) -> i64 {
        todo!()
    }
}
