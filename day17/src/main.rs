use std::{collections::BinaryHeap, fs};

use anyhow::Result;

const FILENAME: &str = "input";

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Eq)]
struct DijkstraTile {
    dist: i32,
    x: usize,
    y: usize,
    dir: Direction,
}

impl Ord for DijkstraTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for DijkstraTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist.cmp(&other.dist).reverse())
    }
}

impl PartialEq for DijkstraTile {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

fn print_arr(arr: &Vec<Vec<i32>>) {
    for l in arr {
        for i in l {
            print!("{} ", i);
        }
        println!("");
    }
    println!("");
}

fn dijkstra(graph: &Vec<Vec<u32>>) -> i32 {
    let mut dist_hor = vec![vec![i32::MAX; graph[0].len()]; graph.len()];
    let mut dist_ver = vec![vec![i32::MAX; graph[0].len()]; graph.len()];

    dist_hor[0][0] = 0;
    dist_ver[0][0] = 0;

    let mut q = BinaryHeap::new();
    q.push(DijkstraTile {
        dist: 0,
        x: 0,
        y: 0,
        dir: Direction::Horizontal,
    });
    q.push(DijkstraTile {
        dist: 0,
        x: 0,
        y: 0,
        dir: Direction::Vertical,
    });

    while !q.is_empty() {
        let tile = q.pop().unwrap();

        // dbg!(&tile);

        // dbg!(dist_hor[tile.y][tile.x], dist_ver[tile.y][tile.x]);

        if (tile.dir == Direction::Horizontal && tile.dist != dist_hor[tile.y][tile.x])
            || (tile.dir == Direction::Vertical && tile.dist != dist_ver[tile.y][tile.x])
        {
            continue;
        }

        let mut adj_tiles = Vec::new();

        match tile.dir {
            Direction::Horizontal => {
                let mut dist = 0;
                for i in (tile.y + 1)..(tile.y + 4).min(graph.len()) {
                    dist += graph[i][tile.x];
                    adj_tiles.push(DijkstraTile {
                        dist: dist as i32,
                        x: tile.x,
                        y: i,
                        dir: Direction::Vertical,
                    });
                }
                dist = 0;
                for i in ((tile.y as i32 - 4).max(0)..(tile.y as i32)).rev() {
                    dist += graph[i as usize][tile.x];
                    adj_tiles.push(DijkstraTile {
                        dist: dist as i32,
                        x: tile.x,
                        y: i as usize,
                        dir: Direction::Vertical,
                    });
                }
            }
            Direction::Vertical => {
                let mut dist = 0;
                for i in (tile.x + 1)..(tile.x + 4).min(graph[0].len()) {
                    dist += graph[tile.y][i];
                    adj_tiles.push(DijkstraTile {
                        dist: dist as i32,
                        x: i,
                        y: tile.y,
                        dir: Direction::Horizontal,
                    });
                }
                dist = 0;
                for i in ((tile.x as i32 - 4).max(0)..(tile.x as i32)).rev() {
                    dist += graph[tile.y][i as usize];
                    adj_tiles.push(DijkstraTile {
                        dist: dist as i32,
                        x: i as usize,
                        y: tile.y,
                        dir: Direction::Horizontal,
                    });
                }
            }
        }

        // dbg!(&adj_tiles);

        for t in adj_tiles {
            let (dist_ref, dist_ref2) = match tile.dir {
                Direction::Horizontal => (&mut dist_ver, &mut dist_hor),
                Direction::Vertical => (&mut dist_hor, &mut dist_ver),
            };


            // dbg!(dist_ref[tile.y][tile.x], dist_ref2[tile.y][tile.x], t.dist);

            if dist_ref2[tile.y][tile.x] + t.dist < dist_ref[t.y][t.x] {
                dist_ref[t.y][t.x] = dist_ref2[tile.y][tile.x] + t.dist;
                q.push(DijkstraTile {
                    dist: dist_ref[t.y][t.x],
                    x: t.x,
                    y: t.y,
                    dir: t.dir,
                });
            }
        }
    }

    // dbg!(&dist_hor, &dist_ver);

    // print_arr(&dist_hor);
    // print_arr(&dist_ver);

    *dist_hor
        .last()
        .unwrap()
        .last()
        .unwrap()
        .min(dist_ver.last().unwrap().last().unwrap())
}

fn main() -> Result<()> {
    let map_graph: Vec<Vec<u32>> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let dist = dijkstra(&map_graph);

    println!("{}", dist);

    Ok(())
}
