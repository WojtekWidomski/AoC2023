use anyhow::{Ok, Result};
use std::fs;

const FILENAME: &str = "test1.in";

fn move_up(platform: &mut Vec<Vec<char>>) {
    for j in 0..platform[0].len() {
        let mut n = 0;
        for i in (0..platform.len()).rev() {
            match platform[i][j] {
                'O' => {
                    n += 1;
                    platform[i][j] = '.';
                }
                '#' => {
                    for i1 in (i + 1)..(i + 1 + n) {
                        platform[i1][j] = 'O';
                    }
                    n = 0;
                }
                _ => {}
            }
        }
        for i in 0..n {
            platform[i][j] = 'O';
        }
    }
}

fn print_platform(platform: &Vec<Vec<char>>) {
    for l in platform {
        for c in l {
            print!("{}", c);
        }
        print!("\n");
    }
}

fn main() -> Result<()> {
    let mut platform: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    move_up(&mut platform);

    let mut sum = 0;

    for i in 0..platform.len() {
        sum += platform[i].iter().filter(|&c| *c == 'O').count() * (platform.len() - i);
    }

    print_platform(&platform);

    println!("{}", sum);

    Ok(())
}
