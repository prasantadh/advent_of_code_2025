use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let (operators, operands) = read_input(filename);
        let mut answers = operands.last().unwrap().clone();
        for operand in operands {
            for (i, val) in operand.iter().enumerate() {
                if operators[i] == Operator::Mult {
                    answers[i] *= val
                } else {
                    answers[i] += val
                }
            }
        }
        answers.iter().sum::<u64>() as i64
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        // INFO: read the input file
        // this problem was different because it needed manipulation of input
        // format itself and could not user read_input
        let lines = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Should have been able to read {} as input file", filename));
        let lines = lines.split("\n").collect::<Vec<&str>>();
        let (_, lines) = lines.split_last().unwrap();

        // INFO: reformat the data such that it is in the format
        // operator
        // operands
        // empty line
        let mut vectical: Vec<String> = vec![];
        for i in 0..lines[0].len() {
            let mut s = String::new();
            let operator = lines[lines.len() - 1].chars().nth(i).unwrap();
            if operator == '+' || operator == '*' {
                vectical.push(String::from(operator));
            }
            for line in lines[0..lines.len() - 1].iter() {
                s.push(line.chars().nth(i).unwrap());
            }
            vectical.push(s);
        }
        vectical.push(String::from(""));

        // INFO: calculate the answer. After the reformatting this one becomes
        // straight-forward
        let mut answer = 0;
        let mut operator = Operator::Add;
        let mut delta = 0;
        for line in vectical {
            match line.trim() {
                "+" => {
                    operator = Operator::Add;
                    delta = 0;
                }
                "*" => {
                    operator = Operator::Mult;
                    delta = 1;
                }
                "" => answer += delta,
                v => {
                    let value = v.parse::<u64>().unwrap();
                    match operator {
                        Operator::Add => delta += value,
                        Operator::Mult => delta *= value,
                    };
                }
            }
        }
        answer as i64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
}

fn read_input(filename: &str) -> (Vec<Operator>, Vec<Vec<u64>>) {
    let operations: Vec<Vec<u64>> = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input file", filename))
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| match v.chars().next().unwrap() {
                    '+' => 0,
                    '*' => 1,
                    _ => v.parse::<u64>().unwrap(),
                })
                .collect::<Vec<u64>>()
        })
        .collect();
    let operators = operations
        .last()
        .unwrap()
        .iter()
        .map(|v| {
            if *v == 0 {
                Operator::Add
            } else {
                Operator::Mult
            }
        })
        .collect::<Vec<Operator>>();
    (operators, operations)
}
