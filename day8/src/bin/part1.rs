use std::{collections::HashMap, fs};

const FILENAME: &str = "input";

fn main() {
    let input_data = fs::read_to_string(FILENAME).unwrap();
    let mut lines = input_data.split("\n");

    let instr = lines.next().unwrap();

    let mut map = HashMap::new();

    for l in lines.skip(1) {
        if l == "" {
            break;
        }

        let start = &l[0..3];

        let left = &l[7..10];
        let right = &l[12..15];

        map.insert(start, (left, right));
    }

    let mut i = 0;
    let mut step = 0;

    let mut node = "AAA";

    while node != "ZZZ" {
        if instr.chars().nth(step).unwrap() == 'L' {
            node = map.get(node).unwrap().0;
        } else {
            node = map.get(node).unwrap().1;
        }

        i += 1;
        step = (step + 1) % instr.len();
    }

    println!("{}", i);
}
