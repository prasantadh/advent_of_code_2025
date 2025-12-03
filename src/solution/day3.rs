use crate::Solution;

use std::cmp::max;
use std::fs;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let banks = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file");
        let banks = banks.trim().split("\n");
        let mut answer = 0;
        for bank in banks {
            let bank: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut tenth = 0;
            for i in 0..(bank.len() - 1) {
                if bank[i] > bank[tenth] {
                    tenth = i;
                }
            }

            let mut ones = tenth + 1;
            for i in (tenth + 1)..bank.len() {
                if bank[i] > bank[ones] {
                    ones = i;
                }
            }

            answer += bank[tenth] * 10 + bank[ones];
        }
        return answer as i64;
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
        // 200 lines of input
        // 100 digits each
        let banks = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file");
        let banks = banks.trim().split("\n");
        let mut answer = 0;
        for bank in banks {
            let mut memo = vec![[0u64; 13]; bank.len()];
            for i in 0..bank.len() {
                for j in 1..=12 {
                    if (i + 1) < j {
                        continue;
                    } else if (i + 1) == j {
                        memo[i][j] = bank[0..=i].parse::<u64>().unwrap();
                    } else {
                        memo[i][j] = max(
                            memo[i - 1][j],
                            memo[i - 1][j - 1] * 10
                                + bank.chars().nth(i).unwrap().to_digit(10).unwrap() as u64,
                        );
                    }
                }
            }
            answer += memo[bank.len() - 1][12];
        }
        return answer as i64;
    }
}
