use std::{fs, mem::swap};

use anyhow::{Ok, Result};

const FILENAME: &str = "test1.in";
const EMPTY_SIZE: i32 = 10; // for test input

// const EMPTY_SIZE: i32 = 1000000;

fn expand(
    image: &mut Vec<Vec<char>>,
    empty_col_prefsum: &mut Vec<i32>,
    empty_row_prefsum: &mut Vec<i32>,
) {
    empty_col_prefsum.resize(image[0].len(), 0);
    empty_row_prefsum.resize(image.len(), 0);

    for i in 0..image.len() {
        let mut empty = true;
        for j in 0..image[i].len() {
            if image[i][j] != '.' {
                empty = false;
                break;
            }
        }
        if i != 0 {
            empty_row_prefsum[i] = empty_row_prefsum[i - 1];
        }
        if empty {
            empty_row_prefsum[i] += 1;
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
        if j != 0 {
            empty_col_prefsum[j] = empty_col_prefsum[j - 1];
        }
        if empty {
            empty_col_prefsum[j] += 1;
        }
    }
}

fn get_from_prefsum(start: usize, end: usize, prefsum: &Vec<i32>) -> i32 {
    let mut start = start;
    let mut end = end;
    if start > end {
        swap(&mut start,&mut end);
    }
    if start == 0 {
        return prefsum[end];
    }
    prefsum[end] - prefsum[start-1]
}

fn path_sum(
    image: &Vec<Vec<char>>,
    empty_col_prefsum: &Vec<i32>,
    empty_row_prefsum: &Vec<i32>,
) -> i32 {
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
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let dist_x = (g1.0 - g2.0).abs()
                + get_from_prefsum(g2.0 as usize, g1.0 as usize, empty_col_prefsum) * EMPTY_SIZE;
            let dist_y = (g1.1 - g2.1).abs()
                + get_from_prefsum(g2.1 as usize, g1.1 as usize, empty_row_prefsum) * EMPTY_SIZE;
            result += dist_x + dist_y;
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

    let mut empty_col_prefsum: Vec<i32> = Vec::new();
    let mut empty_row_prefsum: Vec<i32> = Vec::new();
    expand(&mut space_image, &mut empty_col_prefsum, &mut empty_row_prefsum);

    dbg!(&empty_col_prefsum, &empty_row_prefsum);

    let length_sum = path_sum(&space_image, &empty_col_prefsum, &empty_row_prefsum);

    println!("{}", length_sum);

    Ok(())
}
