use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";
const SLOPES: bool = true; // false for part 2

// This is solution for part 1 and brutal solution for part 2 (too slow)

// Returns vector of tile that can be reached from given tile.
fn get_adj(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    if SLOPES {
        match map[y][x] {
            '^' => up = true,
            'v' => down = true,
            '<' => left = true,
            '>' => right = true,
            _ => {
                up = true;
                down = true;
                left = true;
                right = true;
            }
        }
    } else {
        up = true;
        down = true;
        left = true;
        right = true;
    }

    let mut adj = Vec::new();

    if up && y > 0 && map[y - 1][x] != '#' {
        adj.push((x, y - 1));
    }
    if down && y < map.len() - 1 && map[y + 1][x] != '#' {
        adj.push((x, y + 1));
    }
    if left && x > 0 && map[y][x - 1] != '#' {
        adj.push((x - 1, y));
    }
    if right && x < map.len() - 1 && map[y][x + 1] != '#' {
        adj.push((x + 1, y));
    }

    adj
}

fn dfs(
    x: usize,
    y: usize,
    map: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<bool>>,
    d: i32,
    max_d: &mut i32,
) {
    if y == map.len() - 1 {
        // This is last tile
        *max_d = d.max(*max_d);
        return;
    }

    let adj = get_adj(x, y, map);

    vis[y][x] = true;

    for a in adj {
        if !vis[a.1][a.0] {
            dfs(a.0, a.1, map, vis, d + 1, max_d);
        }
    }

    // Mark tile as not visited again, so all paths will be analyzed.
    vis[y][x] = false;
}

fn main() -> Result<()> {
    let map: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let start_x = map[0].iter().position(|&x| x == '.').unwrap();

    let mut max_length = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    dfs(start_x, 0, &map, &mut visited, 0, &mut max_length);

    println!("{}", max_length);

    Ok(())
}
