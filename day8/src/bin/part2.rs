use std::{collections::HashMap, fs};

const FILENAME: &str = "input";

fn main() {
    let input_data = fs::read_to_string(FILENAME).unwrap();
    let mut lines = input_data.split("\n");

    let instr = lines.next().unwrap();

    let mut map = HashMap::new();

    // Starting node and steps required to get to the ending node
    let mut starting_nodes = HashMap::new();

    for l in lines.skip(1) {
        if l == "" {
            break;
        }

        let start = &l[0..3];

        let left = &l[7..10];
        let right = &l[12..15];

        if start.chars().nth(2).unwrap() == 'A' {
            starting_nodes.insert(start, None);
        }

        map.insert(start, (left, right));
    }

    for (n, d_data) in starting_nodes.iter_mut() {
        let mut i: u128 = 0;
        let mut step = 0;

        let mut node = *n;

        while node.chars().nth(2).unwrap() != 'Z' {
            if instr.chars().nth(step).unwrap() == 'L' {
                node = map.get(node).unwrap().0;
            } else {
                node = map.get(node).unwrap().1;
            }
            i += 1;
            step = (step + 1) % instr.len();
        }
        *d_data = Some(i);
    }

    let mut steps_lcm: u128 = 1;

    for (_n, steps) in starting_nodes {
        steps_lcm = num_integer::lcm(steps_lcm, steps.unwrap());
    }

    println!("{}", steps_lcm);
}
