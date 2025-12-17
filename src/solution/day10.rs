use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let machines = read_input(filename);
        let mut answers_sum = 0;
        for machine in machines {
            let mut answer = machine.buttons.len() + 1;
            // get a mask to enumerate all valid states
            for mask in 0..1 << machine.buttons.len() {
                let mut n_buttons_flipped = 0;
                let mut state = vec![false; machine.indicator.len()];
                // for every button on the mask
                for i in 0..machine.buttons.len() {
                    if 1 << i & mask != 0 {
                        n_buttons_flipped += 1;
                        // flip everything that this button flips
                        for pos in machine.buttons[i].iter() {
                            state[*pos] = !state[*pos];
                        }
                    }
                }
                if state == machine.indicator && n_buttons_flipped < answer {
                    answer = n_buttons_flipped;
                }
            }
            answers_sum += answer;
        }
        answers_sum as i64
    }
}

impl Solution for Part2 {
    fn solve(&self, _filename: &str) -> i64 {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct Machine {
    indicator: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn read_input(filename: &str) -> Vec<Machine> {
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input.", filename));
    let input = input.trim().split("\n");
    let mut machines = vec![];
    for line in input {
        let mut parts = line.trim().split(" ").peekable();
        let indicator = parts
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect::<Vec<bool>>();

        let mut buttons: Vec<Vec<usize>> = vec![];
        loop {
            match parts.peek() {
                Some(v) if v.starts_with('(') => {
                    let button = v
                        .split(',')
                        .filter_map(|s| match s {
                            v if v.starts_with('(') && v.ends_with(')') => {
                                v[1..v.len() - 1].parse::<usize>().ok()
                            }
                            v if v.starts_with('(') => v[1..].parse::<usize>().ok(),
                            v if v.ends_with(')') => v[..v.len() - 1].parse::<usize>().ok(),
                            v => v.parse::<usize>().ok(),
                        })
                        .collect::<Vec<usize>>();
                    buttons.push(button);
                }
                _ => break,
            }
            parts.next();
        }

        let joltage = parts
            .next()
            .unwrap()
            .split(",")
            .filter_map(|s| match s {
                v if v.starts_with('{') => v[1..].parse::<usize>().ok(),
                v if v.ends_with('}') => v[..v.len() - 1].parse::<usize>().ok(),
                v => v.parse::<usize>().ok(),
            })
            .collect::<Vec<usize>>();
        machines.push(Machine {
            indicator,
            buttons,
            joltage,
        });
    }
    machines
}
