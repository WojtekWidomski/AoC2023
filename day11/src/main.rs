use std::fs;

use anyhow::{Ok, Result};

const FILENAME: &str = "input";

fn expand(image: &mut Vec<Vec<char>>) {
    let mut empty_columns = Vec::new();
    let mut empty_rows = Vec::new();

    for i in 0..image.len() {
        let mut empty = true;
        for j in 0..image[i].len() {
            if image[i][j] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_rows.push(i);
        }
    }

    for j in 0..image[0].len() {
        let mut empty = true;
        for i in 0..image.len() {
            if image[i][j] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(j);
        }
    }

    empty_columns.reverse();
    empty_rows.reverse();

    for r in empty_rows {
        image.insert(r, vec!['.'; image[0].len()]);
    }
    for c in empty_columns {
        for line in &mut *image {
            line.insert(c, '.');
        }
    }
}

fn path_sum(image: &Vec<Vec<char>>) -> i32 {
    let mut galaxies = Vec::new();
    let mut result = 0;

    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if image[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            result += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }

    result
}

fn main() -> Result<()> {
    let mut space_image: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    expand(&mut space_image);

    let length_sum = path_sum(&space_image);

    println!("{}", length_sum);

    Ok(())
}
