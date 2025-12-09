use std::cmp::max;
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
    fn solve(&self, _filename: &str) -> i64 {
        todo!()
    }
}

fn read_input(filename: &str) -> Vec<Vertex> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input", filename))
        .trim()
        .split("\n")
        .filter_map(|v| {
            let mut s = v.split(",");
            Some(Vertex(s.next()?.parse().ok()?, s.next()?.parse().ok()?))
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Vertex(u64, u64);
#[derive(Debug, Clone, Copy)]
struct Edge(Vertex, Vertex);
