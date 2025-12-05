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

fn neighbors_count(rows: &Vec<Vec<bool>>, r: usize, c: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter_map(|v| {
            let r = r.checked_add_signed(v.0)?;
            let c = c.checked_add_signed(v.1)?;
            if r >= rows.len() || c >= rows[0].len() || !rows[r][c] {
                return None;
            }
            Some(rows[r][c])
        })
        .count()
}

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let rows = read_input(filename);
        let mut answer = 0;
        for (r, row) in rows.iter().enumerate() {
            for (c, roll) in row.iter().enumerate() {
                // count the number of rolls around by mapping directions into neighbor
                if *roll && neighbors_count(&rows, r, c) < 4 {
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
        let mut rows = read_input(filename);
        let mut answer = 0;
        loop {
            let mut eliminations: Vec<(usize, usize)> = vec![];
            for (r, row) in rows.iter().enumerate() {
                for (c, roll) in row.iter().enumerate() {
                    if *roll && neighbors_count(&rows, r, c) < 4 {
                        eliminations.push((r, c));
                    };
                }
            }

            if eliminations.len() == 0 {
                break;
            }
            answer += eliminations.len();
            for (r, c) in eliminations {
                rows[r][c] = false;
            }
        }
        return answer as i64;
    }
}

fn read_input(filename: &str) -> Vec<Vec<bool>> {
    fs::read_to_string(filename)
        .expect(format!("Should have been able to read {} as input file", filename).as_str())
        .trim()
        .split("\n")
        .map(|v| v.chars().map(|v| v == '@').collect())
        .collect()
}
