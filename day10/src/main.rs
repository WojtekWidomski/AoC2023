use anyhow::{Ok, Result};
use std::fs;

const FILENAME: &str = "input";

// This is type of tile in a loop
#[derive(Clone)]
enum LoopTile {
    Vertical, // It is possible to go down (|, J, 7)
    NotVertical, // It is not possible to go down
    None, // This tile is not part of the loop
}

// Find, what type starting pipe has. This mutates input vector.
fn start_tile_type(pipes: &mut Vec<Vec<char>>, x: usize, y: usize) {
    // left, right, up, down
    let mut l = false;
    let mut r = false;
    let mut u = false;
    let mut d = false;

    // Check, in what directions we can go.
    if y != 0 && (pipes[y - 1][x] == '|' || pipes[y - 1][x] == '7' || pipes[y - 1][x] == 'F') {
        u = true;
    }
    if y != pipes.len() - 1
        && (pipes[y + 1][x] == '|' || pipes[y - 1][x] == 'L' || pipes[y - 1][x] == 'J')
    {
        d = true;
    }
    if x != 0 && (pipes[y][x - 1] == '-' || pipes[y][x - 1] == 'L' || pipes[y][x - 1] == 'F') {
        l = true;
    }
    if x != pipes[0].len() - 1
        && (pipes[y][x + 1] == '-' || pipes[y][x + 1] == '7' || pipes[y][x + 1] == 'J')
    {
        r = true;
    }

    if u && d {
        pipes[y][x] = '|';
        return;
    }
    if l && r {
        pipes[y][x] = '-';
        return;
    }
    if u && r {
        pipes[y][x] = 'L';
        return;
    }
    if u && l {
        pipes[y][x] = 'J';
        return;
    }
    if l && d {
        pipes[y][x] = '7';
        return;
    }
    if r && d {
        pipes[y][x] = 'F';
        return;
    }
}

// Change x and y to next tile.
fn next_tile(x: &mut usize, y: &mut usize, last_x: usize, last_y: usize, pipes: &Vec<Vec<char>>) {
    // Check in what directions it is possible to go and go to tile that is not
    // the last tile.
    match pipes[*y][*x] {
        '|' => {
            if last_y == *y - 1 {
                *y += 1;
            } else {
                *y -= 1;
            }
        }
        '-' => {
            if last_x == *x - 1 {
                *x += 1;
            } else {
                *x -= 1;
            }
        }
        'L' => {
            if last_y == *y - 1 {
                *x += 1;
            } else {
                *y -= 1;
            }
        }
        'J' => {
            if last_y == *y - 1 {
                *x -= 1;
            } else {
                *y -= 1;
            }
        }
        '7' => {
            if last_x == *x - 1 {
                *y += 1;
            } else {
                *x -= 1;
            }
        }
        'F' => {
            if last_x == *x + 1 {
                *y += 1;
            } else {
                *x += 1;
            }
        }
        _ => {}
    }
}

fn main() -> Result<()> {
    let mut input_data: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    // Find starting position ('S' tile)
    let mut start_x = 0;
    let mut start_y = 0;

    for i in 0..input_data.len() {
        for j in 0..input_data[0].len() {
            if input_data[i][j] == 'S' {
                start_x = j;
                start_y = i;
            }
        }
    }

    start_tile_type(&mut input_data, start_x, start_y);

    // Find loop
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut last_x = start_x;
    let mut last_y = start_y;

    let mut loop_length = 0;

    let mut loop_tiles = vec![vec![LoopTile::None; input_data[0].len()]; input_data.len()];

    while current_x != start_x || current_y != start_y || loop_length == 0 {
        let tmp_x = current_x;
        let tmp_y = current_y;
        loop_tiles[current_y][current_x] = match input_data[current_y][current_x] {
            '|' => LoopTile::Vertical,
            'F' => LoopTile::Vertical,
            '7' => LoopTile::Vertical,
            _ => LoopTile::NotVertical,
        };
        next_tile(&mut current_x, &mut current_y, last_x, last_y, &input_data);
        last_x = tmp_x;
        last_y = tmp_y;
        loop_length += 1;
    }

    let max_dist = loop_length / 2;

    println!("max distance: {}", max_dist);

    // Count tiles enclosed in loop
    let mut enclosed = 0;
    let mut count_enabled = false;

    for line in loop_tiles {
        for tile in line {
            match tile {
                LoopTile::Vertical => count_enabled = !count_enabled,
                LoopTile::NotVertical => {}
                LoopTile::None => {
                    if count_enabled {
                        enclosed += 1;
                    }
                }
            }
        }
    }

    println!("tiles enclosed in loop: {}", enclosed);

    Ok(())
}
