use std::cmp::max;
use std::cmp::min;
use std::fs;

use crate::Solution;

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let vertices = read_input(filename);
        let mut answer = 0;
        for i in 0..vertices.len() {
            for j in i + 1..vertices.len() {
                let area = (vertices[i].0.abs_diff(vertices[j].0) + 1)
                    * (vertices[i].1.abs_diff(vertices[j].1) + 1);
                answer = max(answer, area);
            }
        }
        answer as i64
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let vertices = read_input(filename);
        let mut answer = 0;
        for (i, v1) in vertices.iter().enumerate() {
            'v2: for v2 in vertices[i + 1..].iter() {
                // Build a rectangle with corners at v1 and v2
                let top = min(v1.0, v2.0);
                let btm = max(v1.0, v2.0);
                let left = min(v1.1, v2.1);
                let right = max(v1.1, v2.1);

                // if any edge intersects with the rectangle, this is not a valid rectangle
                for (j, this) in vertices.iter().enumerate() {
                    let next = vertices[(j + 1) % vertices.len()];
                    if this.0 == next.0 && (top <= this.0 && this.0 <= btm) {
                        let edge = (this.1.min(next.1), this.1.max(next.1));
                        if (left < edge.0 && edge.0 < right)
                            || (left < edge.1 && edge.1 < right)
                            || (edge.0 < left && right < edge.1)
                        {
                            continue 'v2;
                        }
                    } else if this.1 == next.1 && (left <= this.1 && this.1 <= right) {
                        let edge = (this.0.min(next.0), this.0.max(next.0));
                        if (top < edge.0 && edge.0 < btm)
                            || (top < edge.1 && edge.1 < btm)
                            || (edge.0 < top && btm < edge.1)
                        {
                            continue 'v2;
                        }
                    }
                }
                let area = (v1.0.abs_diff(v2.0) + 1) * (v1.1.abs_diff(v2.1) + 1);
                answer = max(area, answer);
            }
        }
        answer as i64
    }
}

fn read_input(filename: &str) -> Vec<Vertex> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input", filename))
        .trim()
        .split("\n")
        .filter_map(|v| {
            let mut s = v.split(",");
            let col = s.next()?.parse().ok()?; // x coordinate maps to column number
            let row = s.next()?.parse().ok()?; // y coordinate maps to row number
            Some(Vertex(row, col))
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex(u64, u64);
