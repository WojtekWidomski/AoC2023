use std::fs;

// This is solution in Rust. It uses original input.
// It uses the same brutal algorithm, but it is even faster than C++.
// It was written after submitting the answer, only to see how
// this will work in Rust.

const FILENAME: &str = "../input.in";

#[derive(Debug)]
struct MapRange {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

fn read_file(seed_ranges: &mut Vec<(i64, i64)>, maps: &mut Vec<Vec<MapRange>>) {
    let input_file = fs::read_to_string(FILENAME).unwrap();
    let lines: Vec<&str> = input_file.split("\n").collect();

    let first_line = lines[0];

    let mut range_start = -1;
    let mut range_length = -1;

    first_line.split_ascii_whitespace().skip(1).for_each(|s| {
        if range_start == -1 {
            range_start = s.parse().unwrap();
            return;
        }
        range_length = s.parse().unwrap();
        seed_ranges.push((range_start, range_length));
        range_start = -1;
    });

    let mut new_map = true;
    lines.iter().skip(2).for_each(|&l| {
        if new_map {
            maps.push(Vec::new());
            new_map = false;
            return;
        }
        if l == "" {
            new_map = true;
            return;
        }
        let line_numbers: Vec<i64> = l.split(" ").map(|nmb| nmb.parse().unwrap()).collect();

        let maps_length = maps.len();

        maps[maps_length - 1].push(MapRange {
            dest_start: line_numbers[0],
            src_start: line_numbers[1],
            length: line_numbers[2],
        });
    });
}

fn get_from_map(map: &Vec<MapRange>, x: i64) -> i64 {
    for r in map {
        if x >= r.src_start && x < r.src_start + r.length {
            return x + r.dest_start - r.src_start;
        }
    }
    x
}

fn main() {
    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    let mut maps: Vec<Vec<MapRange>> = Vec::new();

    read_file(&mut seed_ranges, &mut maps);

    let mut min_loc = i64::MAX;

    for r in seed_ranges {
        // eprintln!("({}, {})", r.0, r.1);
        for s in r.0..(r.0 + r.1) {
            // if s % 10000000 == 0 {
            //     eprintln!("{}", (100 * (s - r.0)) / r.1);
            // }
            let mut loc = s;
            for m in &maps {
                loc = get_from_map(m, loc);
            }
            if loc < min_loc {
                min_loc = loc;
            }
        }
    }

    println!("{}", min_loc);
}
