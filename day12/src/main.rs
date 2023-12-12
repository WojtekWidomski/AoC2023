use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

fn is_correct(row: &Vec<char>, correct_groups: &Vec<i32>) -> bool {
    let mut groups = Vec::new();

    let mut number = 0;

    for c in row {
        if *c == '#' {
            number += 1;
        } else if number != 0 {
            groups.push(number);
            number = 0;
        }
    }

    if number != 0 {
        groups.push(number);
    }

    if groups.len() != correct_groups.len() {
        return false;
    }

    for i in 0..correct_groups.len() {
        if groups[i] != correct_groups[i] {
            return false;
        }
    }

    true
}

fn possible_combinations(row: &Vec<char>, groups: &Vec<i32>) -> i32 {
    let mut combinations = 0;
    let unknown_num = row.iter().filter(|&x| *x == '?').count();

    for i in 0..1<<unknown_num {
        let mut new_row = Vec::new();
        let mut j =0;
        for c in row {
            if *c == '?' {
                if (i>>j)&1 == 1 {
                    new_row.push('.');
                } else {
                    new_row.push('#');
                }
                j += 1;
            } else {
                new_row.push(*c);
            }
        }
        if is_correct(&new_row, groups) {
            combinations += 1;
        }
    }

    combinations
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let input_lines = input_data
        .split("\n")
        .filter(|&l| l != "");

    let mut combination_sum = 0;

    for l in input_lines {
        let l_split: Vec<&str> = l.split(" ").collect();
        let row: Vec<char> = l_split[0].chars().collect();
        let groups: Vec<i32> = l_split[1].split(",").map(|x| x.parse().unwrap()).collect();

        combination_sum += possible_combinations(&row, &groups);
    }

    println!("{}", combination_sum);

    Ok(())
}
