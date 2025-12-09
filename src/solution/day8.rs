use crate::Solution;

use std::{cmp::Reverse, collections::BinaryHeap, fs};
use union_find::{QuickFindUf, UnionByRank, UnionFind};

pub struct Part1;
pub struct Part2;

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let coords = read_input(filename);

        // get points closest to each other
        let mut distances = BinaryHeap::new();
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let distance = (coords[i].0 as i64 - coords[j].0 as i64).pow(2)
                    + (coords[i].1 as i64 - coords[j].1 as i64).pow(2)
                    + (coords[i].2 as i64 - coords[j].2 as i64).pow(2);
                distances.push((Reverse(distance), i, j));
            }
        }

        // build circuits: UnionFind
        let mut circuits = QuickFindUf::<UnionByRank>::new(coords.len());
        // INFO: This problem is somewhat different than others in that this paramater changes
        // based on the input. for small case this is 10, for large 1000.
        for _ in 0..1000 {
            if let Some((_, i, j)) = distances.pop() {
                circuits.union(i, j);
            };
        }

        // calculate the connected components
        let mut cc: Vec<usize> = vec![0; coords.len()];
        for i in 0..coords.len() {
            cc[circuits.find(i)] += 1;
        }
        cc.sort();
        cc.reverse();
        (cc[0] * cc[1] * cc[2]) as i64
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        let coords = read_input(filename);

        // get points closest to each other
        let mut distances = BinaryHeap::new();
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let distance = (coords[i].0 as i64 - coords[j].0 as i64).pow(2)
                    + (coords[i].1 as i64 - coords[j].1 as i64).pow(2)
                    + (coords[i].2 as i64 - coords[j].2 as i64).pow(2);
                distances.push((Reverse(distance), i, j));
            }
        }

        // build the circuits
        let mut circuits = QuickFindUf::<UnionByRank>::new(coords.len());
        while let Some((_, i, j)) = distances.pop() {
            circuits.union(i, j);
            let mut cc: Vec<usize> = vec![0; coords.len()];
            for i in 0..coords.len() {
                cc[circuits.find(i)] += 1;
            }
            if cc.iter().max().unwrap() == &coords.len() {
                return (coords[i].0 * coords[j].0) as i64;
            }
        }
        0
    }
}

fn read_input(filename: &str) -> Vec<(u64, u64, u64)> {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Should have been able to read {} as input file", filename))
        .trim()
        .split("\n")
        .filter_map(|v| {
            let mut s = v.trim().split(",");
            Some((
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
            ))
        })
        .collect()
}
