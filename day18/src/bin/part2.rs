use std::{cmp::Ordering, collections::BTreeSet, fs};

use anyhow::Result;

const FILENAME: &str = "test1.in";

#[derive(Debug, PartialEq, Eq)]
enum CornerType {
    UL, // Upper, left
    UR, // Upper, right
    DL, // Down, left
    DR, // Down, right
}

#[derive(Eq, Debug)]
struct Point {
    x: i64,
    y: i64,
    c_type: CornerType,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y == other.y {
            if (self.c_type == CornerType::UL || self.c_type == CornerType::UR)
                && (other.c_type == CornerType::DL || other.c_type == CornerType::DR)
            {
                return Ordering::Less;
            }
            return self.x.cmp(&other.x);
        }
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.c_type == other.c_type
    }
}

#[allow(unused)]
fn print_arr(arr: &Vec<Vec<bool>>) {
    for l in arr {
        for i in l {
            if *i {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn get_corner_from_2_points(dir1: char, dir2: char) -> CornerType {
    if (dir1 == 'U' && dir2 == 'R') || (dir1 == 'L' && dir2 == 'D') {
        return CornerType::UL;
    }
    if (dir1 == 'R' && dir2 == 'D') || (dir1 == 'U' && dir2 == 'L') {
        return CornerType::UR;
    }
    if (dir1 == 'D' && dir2 == 'R') || (dir1 == 'L' && dir2 == 'U') {
        return CornerType::DL;
    }
    if (dir1 == 'D' && dir2 == 'L') || (dir1 == 'R' && dir2 == 'U') {
        return CornerType::DR;
    }
    CornerType::UL // this should never happen
}

fn main() -> Result<()> {
    let plan: Vec<(char, i64)> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| {
            let code = l.split(" ").nth(2).unwrap();
            (
                match code.chars().nth(7).unwrap().to_digit(10).unwrap() {
                    0 => 'R',
                    1 => 'D',
                    2 => 'L',
                    3 => 'U',
                    _ => ' ',
                },
                i64::from_str_radix(&code[2..7], 16).unwrap(),
            )
        })
        .collect();

    let mut x = 0;
    let mut y = 0;

    let mut corners = Vec::new();

    corners.push(Point {
        x,
        y,
        c_type: get_corner_from_2_points(plan.last().unwrap().0, plan.first().unwrap().0),
    });

    for win in plan.windows(2) {
        let old_point = win[0];
        let new_point = win[1];

        match old_point.0 {
            'U' => {
                y += old_point.1;
            }
            'R' => {
                x += old_point.1;
            }
            'D' => {
                y -= old_point.1;
            }
            'L' => {
                y += old_point.1;
            }
            _ => {}
        }

        corners.push(Point {
            x,
            y,
            c_type: get_corner_from_2_points(old_point.0, new_point.0),
        });
    }

    corners.sort();

    dbg!(&corners);

    let mut last_y = corners.first().unwrap().y;

    let mut current_corners = BTreeSet::new();

    for p in corners {
        if y == last_y {
            current_corners.insert(p);
        }
    }

    Ok(())
}
