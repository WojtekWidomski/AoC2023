use std::fs;

use anyhow::Result;

const FILENAME: &str = "test1.in";
const STEPS: i32 = 50;

fn main() -> Result<()> {
    let map: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut visited: Vec<Vec<bool>> = map
        .iter()
        .map(|l| l.iter().map(|c| *c == 'S').collect::<Vec<bool>>())
        .collect();

    for _ in 0..STEPS {
        let old_vis = visited.clone();
        visited = vec![vec![false; map[0].len()]; map.len()];

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if old_vis[i][j] {
                    if map[(map.len() + i - 1) % map.len()][j] != '#' {
                        visited[(map.len() + i - 1) % map.len()][j] = true;
                    }
                    if map[(map.len() + i + 1) % map.len()][j] != '#' {
                        visited[(map.len() + i + 1) % map.len()][j] = true;
                    }
                    if map[i][(map.len() + j - 1) % map.len()] != '#' {
                        visited[i][(map.len() + j - 1) % map.len()] = true;
                    }
                    if map[i][(map.len() + j + 1) % map.len()] != '#' {
                        visited[i][(map.len() + j + 1) % map.len()] = true;
                    }
                }
            }
        }
    }

    let mut vis_numner = 0;

    for l in visited {
        vis_numner += l.iter().filter(|&x| *x).count();
    }

    println!("{}", vis_numner);

    Ok(())
}
