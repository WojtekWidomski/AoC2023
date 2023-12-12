use std::fs;

use anyhow::Result;

const FILENAME: &str = "test2.in";

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

fn possible_combinations(row: &Vec<char>, groups: &Vec<i32>) -> i64 {
    let mut combinations = 0;
    let unknown_num = row.iter().filter(|&x| *x == '?').count() as i64;
    let hash_num = row.iter().filter(|&x| *x == '#').count() as i64;
    let hash_sum: i32 = groups.iter().sum();

    // dbg!(unknown_num, (1 as i64)<<unknown_num);

    for i in (0 as i64)..((1 as i64)<<unknown_num as i64) {
        if i.count_ones() as i64 + hash_num != hash_sum as i64 {
            continue;
        }
        let mut new_row = Vec::new();
        let mut j =0;
        for c in row {
            if *c == '?' {
                if (i>>j)&1 == 1 {
                    new_row.push('#');
                } else {
                    new_row.push('.');
                }
                j += 1;
            } else {
                new_row.push(*c);
            }
        }
        if is_correct(&new_row, groups) {
            combinations += 1;
        }

        if i%1000000 == 0 {
            println!("{}/{}", i, (1 as i64)<<unknown_num as i64);
        }
    }

    combinations
}

fn combinations_unfolded(first: i64, second: i64) -> i64 {
    // For some reason this is geometric sequence
    // no, this was not true.
    let q = second/first;
    if second%first != 0 {
        println!("qwerty");
    }
    first * q.pow(4)
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let input_lines = input_data
        .split("\n")
        .filter(|&l| l != "");

    let mut combination_sum = 0;
    let mut unfolded_combination_sum = 0;

    let mut i = 0;

    for l in input_lines {
        dbg!(i);
        let l_split: Vec<&str> = l.split(" ").collect();
        let row: Vec<char> = l_split[0].chars().collect();
        let groups: Vec<i32> = l_split[1].split(",").map(|x| x.parse().unwrap()).collect();
        let c1 = possible_combinations(&row, &groups);

        let row_2_copies = [row.clone(), vec!['?'], row.clone()].concat();
        let groups_2_copies = [groups.clone(), groups.clone()].concat();
        
        
        println!("{}    {}", l, c1);
        // combination_sum += possible_combinations(&row, &groups);
        
        
        combination_sum += c1;
        // let c2 = possible_combinations(&row_2_copies, &groups_2_copies);
        // let c5 = combinations_unfolded(c1, c2);
        // dbg!(c5);
        // unfolded_combination_sum += c5;

        // println!("{}", i);
        i += 1;
    }

    println!("{}", combination_sum);
    println!("unfolded {}", unfolded_combination_sum);

    Ok(())
}
