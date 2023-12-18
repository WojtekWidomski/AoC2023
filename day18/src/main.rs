use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

const ARR_SIZE: usize = 1000;

#[allow(unused)]
fn print_arr(arr: &Vec<Vec<bool>>) {
    for l in arr {
        for i in l {
            if *i {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let plan: Vec<(char, i32)> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| {
            let mut l_spl = l.split(" ");
            (
                l_spl.next().unwrap().chars().next().unwrap(),
                l_spl.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut terrain = vec![vec![false; ARR_SIZE]; ARR_SIZE];

    let mut x = ARR_SIZE / 2;
    let mut y = ARR_SIZE / 2;

    terrain[y][x] = true;

    for (dir, length) in plan {
        match dir {
            'R' => {
                for _ in 0..length {
                    x += 1;
                    terrain[y][x] = true;
                }
            }
            'L' => {
                for _ in 0..length {
                    x -= 1;
                    terrain[y][x] = true;
                }
            }
            'U' => {
                for _ in 0..length {
                    y -= 1;
                    terrain[y][x] = true;
                }
            }
            'D' => {
                for _ in 0..length {
                    y += 1;
                    terrain[y][x] = true;
                }
            }
            _ => {}
        }
    }

    let mut counting_enabled = false;
    let mut result = 0;

    for i in 0..ARR_SIZE {
        for j in 0..ARR_SIZE {
            if terrain[j][i] && terrain[j][i + 1] {
                counting_enabled ^= true;
            }
            if counting_enabled || terrain[j][i] {
                result += 1;
            }
        }
    }

    println!("{}", result);

    Ok(())
}
