use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

// This solution for part 2 converts input to weighted graph, where all intersections and
// start, and end are vertices of it.

fn get_adj(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut adj = Vec::new();

    if y > 0 && map[y - 1][x] != '#' {
        adj.push((x, y - 1));
    }
    if y < map.len() - 1 && map[y + 1][x] != '#' {
        adj.push((x, y + 1));
    }
    if x > 0 && map[y][x - 1] != '#' {
        adj.push((x - 1, y));
    }
    if x < map.len() - 1 && map[y][x + 1] != '#' {
        adj.push((x + 1, y));
    }

    adj
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, i32)>>, vis: &mut Vec<bool>, d: i32, max_d: &mut i32) {
    if v == graph.len() - 1 {
        *max_d = d.max(*max_d);
        return;
    }

    vis[v] = true;

    for u in &graph[v] {
        if !vis[u.0] {
            dfs(u.0, graph, vis, d + u.1, max_d);
        }
    }

    vis[v] = false;
}

fn main() -> Result<()> {
    let map: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    // intersections_numbers are indeces in intersections
    // If intersections[i] = (x, y), then intersections_numbers[y][x] = Some(i)
    let mut intersections = Vec::new(); // intersection coordinates
    let mut intersections_numbers = vec![vec![None; map[0].len()]; map.len()];

    let start_x = map[0].iter().position(|&x| x == '.').unwrap();
    let end_x = map.last().unwrap().iter().position(|&x| x == '.').unwrap();

    intersections.push((start_x, 0));
    intersections_numbers[0][start_x] = Some(0);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let tile_adj = get_adj(j, i, &map);
            if tile_adj.len() > 2 {
                // this is intersection
                intersections_numbers[i][j] = Some(intersections.len());
                intersections.push((j, i));
            }
        }
    }

    intersections_numbers.last_mut().unwrap()[end_x] = Some(intersections.len());
    intersections.push((end_x, map.len() - 1));

    // Intersections adjacency list
    let mut graph_adj = vec![Vec::new(); intersections.len()];

    for i in 0..intersections.len() {
        let (x, y) = intersections[i];
        let intersec_adj = get_adj(x, y, &map);

        for adj in intersec_adj {
            let mut last = (x, y);
            let mut current = adj;
            let mut length = 1;
            while intersections_numbers[current.1][current.0].is_none() {
                let current_adj = get_adj(current.0, current.1, &map);
                let next = current_adj.iter().find(|&x| *x != last).unwrap();
                last = current;
                current = *next;
                length += 1;
            }
            graph_adj[i].push((intersections_numbers[current.1][current.0].unwrap(), length));
        }
    }

    let mut max_length = 0;
    let mut visited = vec![false; graph_adj.len()];

    dfs(0, &graph_adj, &mut visited, 0, &mut max_length);

    println!("{}", max_length);

    Ok(())
}
