use std::cmp::max;
use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

fn init_input(filename: &str) -> (Vec<u64>, Vec<(u64, u64)>) {
    let lines = fs::read_to_string(filename)
        .expect(format!("Should have been able to read {} as input file", filename).as_str());
    let lines = lines.trim().split("\n");
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut candidates: Vec<u64> = vec![];
    for line in lines {
        match line.trim() {
            range if range.contains("-") => {
                let mut numbers = range.split("-");
                let lower = numbers.next().unwrap().parse::<u64>().unwrap();
                let upper = numbers.next().unwrap().parse::<u64>().unwrap();
                ranges.push((lower, upper));
            }
            candidate if candidate.len() > 0 => {
                let candidate = line.parse::<u64>().unwrap();
                candidates.push(candidate);
            }
            _ => (),
        }
    }
    (candidates, ranges)
}

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let (candidates, ranges) = init_input(filename);
        let mut answer = 0;
        for candidate in candidates {
            for range in &ranges {
                if range.0 <= candidate && candidate <= range.1 {
                    answer += 1;
                    break;
                }
            }
        }
        return answer;
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let (_, mut ranges) = init_input(filename);
        ranges.sort();
        let mut ranges: Vec<(u64, u64, bool)> = ranges.iter().map(|v| (v.0, v.1, true)).collect();
        // merge the overlapping ranges
        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                if ranges[i].2 && ranges[i].1 >= ranges[j].0 {
                    ranges[i].1 = max(ranges[i].1, ranges[j].1);
                    ranges[j].2 = false;
                }
            }
        }

        let answer: u64 = ranges.iter().filter(|v| v.2).map(|v| v.1 - v.0 + 1).sum();
        answer as i64
    }
}
