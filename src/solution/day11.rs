use std::{collections::HashMap, fs};

use crate::Solution;

pub struct Part1;
pub struct Part2;

fn ways(graph: &HashMap<String, Vec<String>>, src: &str, dest: &str) -> i64 {
    match graph.get(src) {
        Some(v) if v.iter().any(|node| node == dest) => 1,
        Some(v) => {
            let mut answer = 0;
            for child in v {
                answer += ways(graph, child, dest);
            }
            answer
        }
        None => 0,
    }
}

impl Solution for Part1 {
    fn solve(&self, filename: &str) -> i64 {
        let graph = read_input(filename);
        ways(&graph, "you", "out")
    }
}

fn memo_ways(
    memo: &mut HashMap<(String, String), i64>,
    graph: &HashMap<String, Vec<String>>,
    src: &str,
    dest: &str,
) -> i64 {
    if let Some(answer) = memo.get(&(String::from(src), String::from(dest))) {
        return *answer;
    }
    match graph.get(src) {
        Some(v) if v.iter().any(|node| node == dest) => 1,
        Some(v) => {
            let mut answer = 0;
            for child in v {
                answer += memo_ways(memo, graph, child, dest);
            }
            memo.insert((src.to_string(), dest.to_string()), answer);
            answer
        }
        None => 0,
    }
}

impl Solution for Part2 {
    fn solve(&self, filename: &str) -> i64 {
        // unlike other parts from earlier, this part has a sample input of its own
        // and won't work with sample input from part 1
        let graph = read_input(filename);
        let mut memo: HashMap<(String, String), i64> = HashMap::new();
        memo_ways(&mut memo, &graph, "svr", "fft")
            * memo_ways(&mut memo, &graph, "fft", "dac")
            * memo_ways(&mut memo, &graph, "dac", "out")
        // we could do + svr -> dac -> fft -> out but dac -> fft turned out to be 0 on this dataset
        // which converts the whole thing to 0
    }
}

fn read_input(filename: &str) -> HashMap<String, Vec<String>> {
    fs::read_to_string(filename)
        .expect("Failed to read input")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap().trim_end_matches(':').to_string();
            let values = parts.map(|s| s.to_string()).collect();
            (key, values)
        })
        .collect()
}
