use std::{collections::VecDeque, fs};

use anyhow::Result;

const FILENAME: &str = "input";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_vertical(dir: &Direction) -> bool {
    match dir {
        Direction::Up => true,
        Direction::Down => true,
        _ => false,
    }
}

// Next tile if current tile was empty
fn next_tile(tile: ((i32, i32), Direction)) -> ((i32, i32), Direction) {
    let x = tile.0 .0;
    let y = tile.0 .1;

    let next_position = match tile.1 {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };

    (next_position, tile.1)
}

fn layout_bfs(layout: &Vec<Vec<char>>, start: ((i32, i32), Direction)) -> i32 {
    let mut q = VecDeque::new();
    q.push_back(start);

    let mut visited = vec![vec![false; layout[0].len()]; layout.len()];
    let mut vis_number = 0;

    let mut vis_up = vec![vec![false; layout[0].len()]; layout.len()];
    let mut vis_down = vec![vec![false; layout[0].len()]; layout.len()];
    let mut vis_left = vec![vec![false; layout[0].len()]; layout.len()];
    let mut vis_rigth = vec![vec![false; layout[0].len()]; layout.len()];

    while !q.is_empty() {
        let tile = q.pop_front().unwrap();
        let x = tile.0 .0;
        let y = tile.0 .1;

        if x < 0 || x >= layout[0].len() as i32 || y < 0 || y >= layout.len() as i32 {
            continue;
        }

        let vis_dir = match tile.1 {
            Direction::Up => &mut vis_up[y as usize][x as usize],
            Direction::Down => &mut vis_down[y as usize][x as usize],
            Direction::Left => &mut vis_left[y as usize][x as usize],
            Direction::Right => &mut vis_rigth[y as usize][x as usize],
        };

        if *vis_dir {
            continue;
        }

        *vis_dir = true;

        if !visited[y as usize][x as usize] {
            visited[y as usize][x as usize] = true;
            vis_number += 1;
        }

        match layout[y as usize][x as usize] {
            '\\' => {
                let new_tile = match tile.1 {
                    Direction::Up => ((x - 1, y), Direction::Left),
                    Direction::Down => ((x + 1, y), Direction::Right),
                    Direction::Left => ((x, y - 1), Direction::Up),
                    Direction::Right => ((x, y + 1), Direction::Down),
                };
                q.push_back(new_tile);
            }
            '/' => {
                let new_tile = match tile.1 {
                    Direction::Up => ((x + 1, y), Direction::Right),
                    Direction::Down => ((x - 1, y), Direction::Left),
                    Direction::Left => ((x, y + 1), Direction::Down),
                    Direction::Right => ((x, y - 1), Direction::Up),
                };
                q.push_back(new_tile);
            }
            '-' => match is_vertical(&tile.1) {
                true => {
                    q.push_back(((x - 1, y), Direction::Left));
                    q.push_back(((x + 1, y), Direction::Right));
                }
                false => q.push_back(next_tile(tile)),
            },
            '|' => match is_vertical(&tile.1) {
                true => q.push_back(next_tile(tile)),
                false => {
                    q.push_back(((x, y - 1), Direction::Up));
                    q.push_back(((x, y + 1), Direction::Down));
                }
            },
            _ => q.push_back(next_tile(tile)),
        }
    }

    vis_number
}

fn main() -> Result<()> {
    let layout: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    // Part 1
    let visited_tiles = layout_bfs(&layout, ((0, 0), Direction::Right));
    println!("Part 1: {}", visited_tiles);

    // Part 2
    let mut max_visited = 0;
    for x in 0..layout[0].len() {
        let from_up = layout_bfs(&layout, ((x as i32, 0), Direction::Down));
        let from_bottom = layout_bfs(
            &layout,
            ((x as i32, layout.len() as i32 - 1), Direction::Up),
        );
        max_visited = max_visited.max(from_bottom).max(from_up);
    }
    for y in 0..layout.len() {
        let from_left = layout_bfs(&layout, ((0, y as i32), Direction::Right));
        let from_right = layout_bfs(
            &layout,
            ((layout[0].len() as i32 - 1, y as i32), Direction::Left),
        );
        max_visited = max_visited.max(from_left).max(from_right);
    }

    println!("Part 2: {}", max_visited);

    Ok(())
}
