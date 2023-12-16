use anyhow::{Ok, Result};
use std::{collections::HashMap, fs};

const FILENAME: &str = "input";

const PART2: bool = true;
const CYCLES: usize = 1000000000;

#[derive(Debug)]
enum TiltDirection {
    Up,
    Down,
    Left,
    Right,
}

fn tilt_platform(platform: &mut Vec<Vec<char>>, direction: TiltDirection) {
    let vertical = match direction {
        TiltDirection::Up => true,
        TiltDirection::Down => true,
        _ => false,
    };

    if vertical {
        for j in 0..platform[0].len() {
            let mut n = 0;
            let iter: Box<dyn Iterator<Item = usize>> = match direction {
                TiltDirection::Up => Box::new((0..platform.len()).rev()),
                _ => Box::new(0..platform.len()),
            };
            for i in iter {
                match platform[i][j] {
                    'O' => {
                        n += 1;
                        platform[i][j] = '.';
                    }
                    '#' => {
                        match direction {
                            TiltDirection::Up => {
                                for i1 in (i + 1)..(i + 1 + n) {
                                    platform[i1][j] = 'O';
                                }
                            }
                            _ => {
                                for i1 in (i - n)..(i) {
                                    platform[i1 as usize][j] = 'O';
                                }
                            }
                        }
                        n = 0;
                    }
                    _ => {}
                }
            }
            match direction {
                TiltDirection::Up => {
                    for i in 0..n {
                        platform[i][j] = 'O';
                    }
                }
                _ => {
                    for i in (platform.len() - n)..platform.len() {
                        platform[i][j] = 'O';
                    }
                }
            }
        }
    } else {
        for i in 0..platform.len() {
            let mut n = 0;
            let iter: Box<dyn Iterator<Item = usize>> = match direction {
                TiltDirection::Left => Box::new((0..platform[0].len()).rev()),
                _ => Box::new(0..platform[0].len()),
            };
            for j in iter {
                match platform[i][j] {
                    'O' => {
                        n += 1;
                        platform[i][j] = '.';
                    }
                    '#' => {
                        match direction {
                            TiltDirection::Left => {
                                for j1 in (j + 1)..(j + 1 + n) {
                                    platform[i][j1] = 'O';
                                }
                            }
                            _ => {
                                for j1 in (j - n)..(j) {
                                    platform[i][j1 as usize] = 'O';
                                }
                            }
                        }
                        n = 0;
                    }
                    _ => {}
                }
            }
            match direction {
                TiltDirection::Left => {
                    for j in 0..n {
                        platform[i][j] = 'O';
                    }
                }
                _ => {
                    for j in (platform[0].len() - n)..platform[0].len() {
                        platform[i][j] = 'O';
                    }
                }
            }
        }
    }
}

fn tilt_cycle(platform: &mut Vec<Vec<char>>) {
    tilt_platform(platform, TiltDirection::Up);
    tilt_platform(platform, TiltDirection::Left);
    tilt_platform(platform, TiltDirection::Down);
    tilt_platform(platform, TiltDirection::Right);
}

#[allow(dead_code)]
fn print_platform(platform: &Vec<Vec<char>>) {
    for l in platform {
        for c in l {
            print!("{}", c);
        }
        print!("\n");
    }
    print!("\n")
}

fn total_load(platform: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for i in 0..platform.len() {
        sum += platform[i].iter().filter(|&c| *c == 'O').count() * (platform.len() - i);
    }

    sum
}

fn main() -> Result<()> {
    let mut platform: Vec<Vec<char>> = fs::read_to_string(FILENAME)?
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    if PART2 {
        let mut possible_states = HashMap::new();
        let mut states_order = Vec::new();
        let mut i = 0;

        while !possible_states.contains_key(&platform) {
            possible_states.insert(platform.clone(), i);
            states_order.push(platform.clone());
            tilt_cycle(&mut platform);
            i += 1;
        }

        let repeat_start = *possible_states.get(&platform).unwrap();

        println!(
            "{}",
            total_load(
                &states_order
                    [(CYCLES - repeat_start) % (states_order.len() - repeat_start) + repeat_start]
            )
        );
    } else {
        tilt_platform(&mut platform, TiltDirection::Up);
        println!("{}", total_load(&platform));
    }

    Ok(())
}
