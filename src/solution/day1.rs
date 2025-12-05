use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

pub struct Action {
    direction: i64,
    magnitude: i64,
}

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let actions = read_input(filename);
        let mut position = 50;
        let mut answer = 0;
        for action in actions {
            position += action.direction * action.magnitude;
            position %= 100;
            answer += (position == 0) as i64
        }
        return answer;
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let actions = read_input(filename);
        let mut position = 50;
        let mut answer = 0;
        for mut action in actions {
            while action.magnitude != 0 {
                position = (position + action.direction) % 100;
                action.magnitude -= 1;
                answer += (position == 0) as i64
            }
        }
        return answer;
    }
}

fn read_input(filename: &str) -> Vec<Action> {
    let actions = fs::read_to_string(filename)
        .expect(format!("Should have been able to read {} as input file", filename).as_str());
    let actions = actions.trim().split("\n");
    actions
        .filter_map(|action| match action.trim().chars().nth(0) {
            Some(v) => Some(Action {
                direction: if v == 'L' { -1 } else { 1 },
                magnitude: action[1..].parse().ok()?,
            }),
            _ => None,
        })
        .collect()
}
