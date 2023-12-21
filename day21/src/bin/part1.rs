use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";
const STEPS: i32 = 6;

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
                    if i > 0 && map[i - 1][j] != '#' {
                        visited[i - 1][j] = true;
                    }
                    if i < map.len() - 1 && map[i + 1][j] != '#' {
                        visited[i + 1][j] = true;
                    }
                    if j > 0 && map[i][j - 1] != '#' {
                        visited[i][j - 1] = true;
                    }
                    if j < map[0].len() - 1 && map[i][j + 1] != '#' {
                        visited[i][j + 1] = true;
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
