use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

fn extrapolate(seq: &Vec<i32>, backwards: bool) -> i32 {
    let mut diff_seqs = Vec::new();

    diff_seqs.push(seq.clone());

    if backwards {
        diff_seqs.first_mut().unwrap().reverse();
    }

    let mut all_0 = false;

    // Compute sequences of differences
    while !all_0 {
        let last_seq = diff_seqs.last().unwrap();
        let mut new_seq = Vec::new();
        all_0 = true;
        for i in 1..last_seq.len() {
            let mut diff = last_seq[i] - last_seq[i - 1];
            if backwards {
                diff *= -1;
            }

            new_seq.push(diff);
            if diff != 0 {
                all_0 = false;
            }
        }
        diff_seqs.push(new_seq);
    }

    diff_seqs.last_mut().unwrap().push(0);

    let mut last_diff = 0;

    // Extrapolate sequences
    for s in diff_seqs.iter_mut().rev().skip(1) {
        let new_diff = s.last().unwrap() + last_diff;
        s.push(new_diff);
        last_diff = new_diff;
        if backwards {
            last_diff *= -1;
        }
    }

    *diff_seqs.first().unwrap().last().unwrap()
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;

    let sequences: Vec<Vec<i32>> = input_data
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| {
            l.split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut extr_val_sum = 0;
    let mut extr_val_sum_back = 0;

    for s in sequences {
        extr_val_sum += extrapolate(&s, false);
        extr_val_sum_back += extrapolate(&s, true);
    }

    println!("sum: {}\nsum backwards: {}", extr_val_sum, extr_val_sum_back);

    Ok(())
}
