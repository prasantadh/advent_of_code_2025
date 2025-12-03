use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let actions = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file");
        let actions = actions.split("\n");
        let mut answer: i64 = 0;
        let mut position: i64 = 50;
        for action in actions {
            position += match action.chars().nth(0) {
                Some('L') => -1 * action[1..].parse::<i64>().unwrap(),
                Some('R') => action[1..].parse::<i64>().unwrap(),
                _ => 0,
            };
            position %= 100;
            if position == 0 {
                answer += 1;
            }
        }
        return answer;
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let actions = fs::read_to_string(filename)
            .expect(format!("Should have been able to read {filename} as input file").as_str());
        let actions = actions.split("\n");
        let mut answer: i64 = 0;
        let mut position: i64 = 50;
        for action in actions {
            let delta = match action.chars().nth(0) {
                Some('L') => -1 * action[1..].parse::<i64>().unwrap(),
                Some('R') => action[1..].parse::<i64>().unwrap(),
                _ => 0,
            };
            match delta {
                mut v if delta < 0 => {
                    while v != 0 {
                        position = (position - 1) % 100;
                        v += 1;
                        answer += if position == 0 { 1 } else { 0 };
                    }
                }
                mut v if v > 0 => {
                    while v != 0 {
                        position = (position + 1) % 100;
                        v -= 1;
                        answer += if position == 0 { 1 } else { 0 };
                    }
                }
                _ => (),
            }
        }
        return answer;
    }
}
