use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Brick {
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by lower z
        self.z1.min(self.z2).cmp(&other.z1.min(other.z2))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    fn is_horizontaly_intersecting(&self, second_brick: &Brick) -> bool {
        let ul_corner = (self.x1.min(self.x2), self.y1.min(self.y2));
        let dr_corner = (self.x1.max(self.x2), self.y1.max(self.y2));
        let second_ul = (
            second_brick.x1.min(second_brick.x2),
            second_brick.y1.min(second_brick.y2),
        );
        let second_dr = (
            second_brick.x1.max(second_brick.x2),
            second_brick.y1.max(second_brick.y2),
        );

        (second_dr.0 >= ul_corner.0 && second_dr.1 >= ul_corner.1)
            && (second_ul.0 <= dr_corner.0 && second_ul.1 <= dr_corner.1)
    }

    fn is_supported_by(&self, second_brick: &Brick) -> bool {
        second_brick.z1.max(second_brick.z2) == self.z1.min(self.z2) - 1
            && self.is_horizontaly_intersecting(second_brick)
    }
}

// Simulate falling of bricks and return number of bricks, that changed thier position.
// bricks must be sorted.
fn fall(bricks: &mut Vec<Brick>) -> i32 {
    let mut falling_number = 0;

    for i in 0..bricks.len() {
        let mut new_min_z = 0; // New down z of brick
        let old_min_z = bricks[i].z1.min(bricks[i].z2); // Old down z of brick
        for j in 0..i {
            let other_brick = &bricks[j];
            if bricks[i].is_horizontaly_intersecting(other_brick) {
                new_min_z = other_brick.z1.max(other_brick.z2).max(new_min_z);
            }
        }
        new_min_z += 1;

        if new_min_z != old_min_z {
            falling_number += 1;
        }

        bricks[i].z1 -= old_min_z - new_min_z;
        bricks[i].z2 -= old_min_z - new_min_z;
    }

    falling_number
}

fn safe_to_remove_number(bricks: &Vec<Brick>) -> i32 {
    let mut can_be_removed = vec![true; bricks.len()];
    for i in 0..bricks.len() {
        let mut last_supported_by_idx = 0;
        let mut support_number = 0;
        for j in 0..bricks.len() {
            if bricks[i].is_supported_by(&bricks[j]) {
                last_supported_by_idx = j;
                support_number += 1;
            }
        }
        if support_number == 1 {
            // If there is only one brick supporting this brick, then last found
            // supporting brick will be this only brick.
            // If support_number == 0, then there are no bricks supporting this brick
            // and this brick is on the ground.
            can_be_removed[last_supported_by_idx] = false;
        }
    }

    can_be_removed.iter().filter(|&x| *x).count() as i32
}

fn main() -> Result<()> {
    let mut bricks: Vec<Brick> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| {
            let l_spl: Vec<&str> = l.split("~").collect();
            let point1: Vec<i32> = l_spl[0].split(",").map(|x| x.parse().unwrap()).collect();
            let point2: Vec<i32> = l_spl[1].split(",").map(|x| x.parse().unwrap()).collect();

            Brick {
                x1: point1[0],
                y1: point1[1],
                z1: point1[2],
                x2: point2[0],
                y2: point2[1],
                z2: point2[2],
            }
        })
        .collect();

    bricks.sort();

    // Part 1
    fall(&mut bricks);

    let safe = safe_to_remove_number(&bricks);

    println!("Part 1: {}", safe);

    // Part 2
    let mut falling_sum = 0;

    // It is probably possible to do it much faster (this is O(n^3)),
    // but there are only 1485 bricks in input.
    for i in 0..bricks.len() {
        let mut bricks_without_i = bricks.clone();
        bricks_without_i.remove(i);
        falling_sum += fall(&mut bricks_without_i);
    }

    println!("Part 2: {}", falling_sum);

    Ok(())
}
