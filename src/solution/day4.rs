use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let rows: Vec<Vec<bool>> = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file")
            .trim()
            .split("\n")
            .map(|v| v.chars().map(|v| v == '@').collect())
            .collect();

        let mut answer = 0;
        for (r, row) in rows.iter().enumerate() {
            for (c, roll) in row.iter().enumerate() {
                // count the number of rolls around by mapping directions into neighbor
                if *roll
                    && DIRECTIONS
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
    /* the fundamental strategy is same as part 1.
     * Here we keep track of all the coordinates that can be removed.
     * Then we remove them, add them to the answer and loop
     * until there is a round where there are no more to remove
     */
    fn solve(&self, filename: &str) -> i64 {
        let mut rows: Vec<Vec<bool>> = fs::read_to_string(filename)
            .expect("Should have been able to read day1.txt as input file")
            .trim()
            .split("\n")
            .map(|v| v.chars().map(|v| v == '@').collect())
            .collect();

        let mut answer = 0;
        loop {
            let mut eliminations: Vec<(usize, usize)> = vec![];
            let mut delta = 0;
            for (r, row) in rows.iter().enumerate() {
                for (c, roll) in row.iter().enumerate() {
                    // count the number of rolls around by mapping directions into neighbor
                    if *roll
                        && DIRECTIONS
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
                        eliminations.push((r, c));
                        delta += 1;
                    };
                }
            }

            if delta == 0 {
                break;
            }
            answer += delta;
            for (r, c) in eliminations {
                rows[r][c] = false;
            }
        }
        return answer;
    }
}
