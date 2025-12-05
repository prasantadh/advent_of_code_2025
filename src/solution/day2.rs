use crate::Solution;

use std::{collections::HashSet, fs};

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let ranges = read_input(filename);
        let mut answer = 0;
        for range in ranges {
            for i in range.0..=range.1 {
                let s = i.to_string();
                if s[0..(s.len() / 2)] == s[(s.len() / 2)..] {
                    answer += i
                }
            }
        }
        answer as i64
    }
}

fn has_repetition(candidate: u64, size: usize) -> bool {
    let candidate = candidate.to_string();
    for i in (size..candidate.len()).step_by(size) {
        if (i + size) > candidate.len() {
            return false;
        }
        if candidate[i..(i + size)] != candidate[0..size] {
            return false;
        }
    }
    return true;
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let ranges = read_input(filename);

        let mut answer: u64 = 0;
        let mut numbers: HashSet<u64> = HashSet::new();
        // TODO: See if there is a better algorithm for this
        for range in ranges {
            for i in range.0..=range.1 {
                for j in 1..=(i.to_string().len() / 2) {
                    if !numbers.contains(&i) && has_repetition(i, j) {
                        numbers.insert(i);
                        answer += i;
                    }
                }
            }
        }
        answer as i64 // substrings of all sizes should be different
    }
}

fn read_input(filename: &str) -> Vec<(u64, u64)> {
    let ranges = fs::read_to_string(filename)
        .expect(format!("Should have been able to read {} as input file", filename).as_str());
    let ranges = ranges.trim().split(",");
    ranges
        .filter_map(|range| {
            let mut range = range.split("-");
            let lower = range.next()?.parse::<u64>().ok()?;
            let upper = range.next()?.parse::<u64>().ok()?;
            Some((lower, upper))
        })
        .collect()
}
