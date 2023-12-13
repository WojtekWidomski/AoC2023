use std::fs;

use anyhow::{Ok, Result};

const FILENAME: &str = "input";

// Convert pattern lines to vectors, where each number represents line
// Each bit in number represents character.
fn lines_to_numbers(pat: &Vec<Vec<char>>, vertical: &mut Vec<i32>, horizontal: &mut Vec<i32>) {
    for i in 0..pat.len() {
        let mut line_num = 0;
        for j in 0..pat[0].len() {
            if pat[i][j] == '#' {
                line_num |= 1 << j;
            }
        }
        horizontal.push(line_num);
    }

    for j in 0..pat[0].len() {
        let mut line_num = 0;
        for i in 0..pat.len() {
            if pat[i][j] == '#' {
                line_num |= 1 << i;
            }
        }
        vertical.push(line_num);
    }
}

// Find perfect reflection and return number of lines before it
// If not found return 0.
fn perfect_reflection(line: &Vec<i32>, smudge: bool) -> i32 {
    let mut result = 0;

    // i is index of number that is on the right side of symmetry line
    for i in 1..line.len() {
        let mut refl_len = 0;
        let mut refl_found = true;
        let mut smudge_found = false;
        while (i as i32 - 1 - refl_len as i32) >= 0 && i + refl_len < line.len() {
            if line[i + refl_len] != line[i - refl_len - 1] {
                if smudge
                    && !smudge_found
                    && (line[i + refl_len] ^ line[i - refl_len - 1]).count_ones() == 1
                {
                    smudge_found = true;
                } else {
                    refl_found = false;
                    break;
                }
            }
            refl_len += 1;
        }
        if refl_found && (!smudge || (smudge && smudge_found)) {
            result = i as i32;
        }
    }
    result
}

fn main() -> Result<()> {
    let patterns: Vec<Vec<Vec<char>>> = fs::read_to_string(FILENAME)?
        .split("\n\n")
        .map(|p| {
            p.split("\n")
                .filter(|&l| l != "")
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    let mut result = 0;
    let mut result_part2 = 0;

    for p in patterns {
        let mut vert_lines = Vec::new();
        let mut hor_lines = Vec::new();

        lines_to_numbers(&p, &mut vert_lines, &mut hor_lines);

        let vert_refl = perfect_reflection(&vert_lines, false);
        let hor_refl = perfect_reflection(&hor_lines, false);

        let vert_refl2 = perfect_reflection(&vert_lines, true);
        let hor_refl2 = perfect_reflection(&hor_lines, true);

        result += vert_refl + 100 * hor_refl;
        result_part2 += vert_refl2 + 100 * hor_refl2;
    }

    println!("part 1 {}", result);
    println!("part 2 {}", result_part2);

    Ok(())
}
