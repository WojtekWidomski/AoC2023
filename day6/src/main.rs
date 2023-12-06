use std::fs;

// This works for both subtasks, because input is very short
// and it was easier to manually edit it.

const FILENAME: &str = "input2.in";

fn win_ways(t: i64, s: i64) -> i64 {
    let mut result = 0;
    for i in 1..t {
        if (t-i)*i > s {
            result += 1;
        }
    }
    result
}

fn main() {
    let input_data = fs::read_to_string(FILENAME).unwrap();
    let input_lines: Vec<&str> = input_data.split("\n").collect();

    let mut times: Vec<i64> = Vec::new();
    let mut dists: Vec<i64> = Vec::new();

    input_lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .for_each(|t| times.push(t.parse().unwrap()));
    input_lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .for_each(|t| dists.push(t.parse().unwrap()));

    let mut result = 1;

    for i in 0..times.len() {
        result *= win_ways(times[i], dists[i]);
    }

    println!("{}", result);
}
