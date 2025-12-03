use crate::Solution;

use std::{collections::HashSet, fs};

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let ranges = fs::read_to_string(filename)
            .expect(format!("Should have been able to read {} as input file", filename).as_str());
        let ranges = ranges.trim().split(",");

        let mut answer: i64 = 0;
        for range in ranges {
            let range: Vec<&str> = range.trim().split('-').collect();
            let lower = range[0].parse::<i64>().unwrap();
            let upper = range[1].parse::<i64>().unwrap();
            for i in lower..=upper {
                let s = i.to_string();
                if s[0..(s.len() / 2)] == s[(s.len() / 2)..] {
                    answer += i
                }
            }
        }
        return answer;
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
        let ranges = fs::read_to_string(filename)
            .expect(format!("Should have been able to read {} as input file", filename).as_str());
        let ranges = ranges.trim().split(",");

        let mut answer: i64 = 0;
        let mut numbers: HashSet<i64> = HashSet::new();
        for range in ranges {
            let range: Vec<&str> = range.trim().split('-').collect();
            let lower = range[0].parse::<i64>().unwrap();
            let upper = range[1].parse::<i64>().unwrap();
            for i in lower..=upper {
                for j in 1..=(i.to_string().len() / 2) {
                    if !numbers.contains(&i) && has_repetition(i as u64, j) {
                        //println!("{} {}", i, j);
                        numbers.insert(i);
                        answer += i;
                    }
                }
            }
        }
        return answer; // substrings of all sizes should be different
    }
}
