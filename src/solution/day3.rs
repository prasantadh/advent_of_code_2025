use crate::Solution;

use std::cmp::max;
use std::fs;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let banks = read_input(filename);
        let mut answer = 0;
        for bank in banks {
            // largest digit for the tenth place in [0..n-1)
            let mut tenth = 0;
            for i in 0..(bank.len() - 1) {
                if bank[i] > bank[tenth] {
                    tenth = i;
                }
            }

            // largest digit for the ones place in [0..n)
            let mut ones = tenth + 1;
            for i in (tenth + 1)..bank.len() {
                if bank[i] > bank[ones] {
                    ones = i;
                }
            }

            answer += bank[tenth] * 10 + bank[ones];
        }
        answer as i64
    }
}

impl Solution for Part2 {
    /*
     * Let x [1..=n] be the bank of batteries.
     * Let f(i, j) be the joltage using exactly j batteries
     * from the subset bank[1..=i]. Then,
     * f(i, j)  = -INF                              for all j > i
     *          = x[1..i]                           for all j = i
     *          = max(  f(i-1, j),
     *                  f(i-1, j-1) * 10 + x[i]
     *          )                                   for all j < i
     */
    fn solve(&self, filename: &str) -> i64 {
        let banks = read_input(filename);
        let mut answer = 0;
        for bank in banks {
            let mut memo = vec![[0u64; 13]; bank.len()];
            for i in 0..bank.len() {
                for j in 1..=12 {
                    if (i + 1) < j {
                        continue;
                    } else if (i + 1) == j {
                        memo[i][j] = digits_to_u64(&bank[0..=i]);
                    } else {
                        memo[i][j] = max(memo[i - 1][j], memo[i - 1][j - 1] * 10 + bank[i] as u64);
                    }
                }
            }
            answer += memo[bank.len() - 1][12];
        }
        answer as i64
    }
}

fn digits_to_u64(slice: &[u32]) -> u64 {
    let mut answer: u64 = 0;
    for v in slice {
        answer = answer * 10 + *v as u64;
    }
    answer
}

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .expect(&*format!(
            "Should have been able to read {} as input file",
            filename
        ))
        .trim()
        .split("\n")
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
