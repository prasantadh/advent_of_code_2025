use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let rows: Vec<Vec<bool>> = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file")
            .trim()
            .split("\n")
            .map(|v| v.chars().map(|v| v == '@').collect())
            .collect();

        let directions: [(isize, isize); 8] = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        let mut answer = 0;
        for (r, row) in rows.iter().enumerate() {
            for (c, roll) in row.iter().enumerate() {
                if !roll {
                    continue;
                }

                // count the number of rolls around by mapping directions into neighbor
                if directions
                    .iter()
                    .map(|v| {
                        let r = r.checked_add_signed(v.0)?;
                        let c = c.checked_add_signed(v.1)?;
                        if r >= rows.len() || c >= rows[0].len() {
                            return None;
                        }
                        Some(rows[r][c])
                    })
                    .filter_map(|v| v)
                    .filter(|v| *v)
                    .count()
                    < 4
                {
                    answer += 1;
                };
            }
        }
        return answer;
    }
}

impl Solution for Part2 {
    fn solve(&self, _filename: &str) -> i64 {
        todo!()
    }
}
