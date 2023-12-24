use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";
// const MIN_POS: f64 = 7.0;
// const MAX_POS: f64 = 27.0;
const MIN_POS: f64 = 200000000000000.0;
const MAX_POS: f64 = 400000000000000.0;

#[derive(Debug)]
struct HailStone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl HailStone {
    fn intersection(&self, second: &HailStone) -> Option<(f64, f64)> {
        let mut a1 = None;
        let mut a2 = None;

        if self.velocity.0 != 0.0 {
            a1 = Some(self.velocity.1 / self.velocity.0);
        }
        if second.velocity.0 != 0.0 {
            a2 = Some(second.velocity.1 / second.velocity.0);
        }

        if a1.is_none() && a2.is_none() {
            // Lines are vertical
            return None;
        }

        if a1.is_none() {
            let b2 = second.position.1 - a2.unwrap() * second.position.0;
            let x = self.position.0;
            let y = a2.unwrap() * x + b2;
            if self.is_in_future((x, y)) && second.is_in_future((x, y)) {
                return Some((x, y));
            } else {
                return None;
            }
        }
        if a2.is_none() {
            let b1 = self.position.1 - a1.unwrap() * self.position.0;
            let x = second.position.0;
            let y = a1.unwrap() * x + b1;
            if self.is_in_future((x, y)) && second.is_in_future((x, y)) {
                return Some((x, y));
            } else {
                return None;
            }
        }

        if a1.unwrap() == a2.unwrap() {
            return None;
        }

        let x1 = self.position.0;
        let y1 = self.position.1;
        let x2 = second.position.0;
        let y2 = second.position.1;

        let b1 = y1 - a1.unwrap() * x1;

        let x = (y2 - a2.unwrap() * x2 - y1 + a1.unwrap() * x1) / (a1.unwrap() - a2.unwrap());
        let y = a1.unwrap() * x + b1;

        if self.is_in_future((x, y)) && second.is_in_future((x, y)) {
            return Some((x, y));
        } else {
            return None;
        }
    }

    fn is_in_future(&self, point: (f64, f64)) -> bool {
        let x_future = (point.0 - self.position.0) * self.velocity.0 > 0.0;
        let y_future = (point.1 - self.position.1) * self.velocity.1 > 0.0;

        x_future && y_future
    }
}

fn main() -> Result<()> {
    let hailstones: Vec<HailStone> = fs::read_to_string(FILENAME)?
        .split_terminator("\n")
        .map(|l| {
            let l_spl: Vec<&str> = l.split(" @ ").collect();
            let pos_data: Vec<f64> = l_spl[0]
                .split(", ")
                .map(|x| x.trim_start().parse().unwrap())
                .collect();
            let v_data: Vec<f64> = l_spl[1]
                .split(", ")
                .map(|x| x.trim_start().parse().unwrap())
                .collect();
            HailStone {
                position: (pos_data[0], pos_data[1], pos_data[2]),
                velocity: (v_data[0], v_data[1], v_data[2]),
            }
        })
        .collect();

    let mut result = 0;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let intersection = hailstones[i].intersection(&hailstones[j]);
            if let Some((x, y)) = intersection {
                if x >= MIN_POS && x <= MAX_POS && y >= MIN_POS && y <= MAX_POS {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);

    Ok(())
}
