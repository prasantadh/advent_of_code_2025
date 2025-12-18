use crate::Solution;

use std::cmp::min;
use std::fs;
use z3::{Solver, ast::Int};

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
    fn solve(&self, filename: &str) -> i64 {
        // takes about a couple minutes to solve it
        let machines = read_input(filename);
        let mut answer = 0;
        for machine in machines {
            // solve for joltage using the linear equations
            let solver = Solver::new();
            let variables: Vec<Int> = (0..machine.buttons.len())
                .map(|i| Int::fresh_const(&format!("x{}", i)))
                .collect();
            for variable in &variables {
                solver.assert(variable.ge(0));
            }
            for (pos, joltage) in machine.joltage.iter().enumerate() {
                let mut expr = Int::from_u64(0);
                for (i, button) in machine.buttons.iter().enumerate() {
                    if button.contains(&pos) {
                        expr += &variables[i];
                    }
                }
                solver.assert((expr).eq(*joltage as u64));
            }
            let mut local = 100000;
            for solution in solver.solutions(variables, false).take(10000) {
                local = min(
                    local,
                    solution.iter().map(|v| (*v).as_i64().unwrap()).sum::<i64>(),
                );
            }
            answer += local;
        }
        answer
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
