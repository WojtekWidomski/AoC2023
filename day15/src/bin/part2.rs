use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

enum CommandType {
    Remove,
    Add(i32),
}

struct Command<'a> {
    box_number: usize,
    lens_label: &'a str,
    command_type: CommandType,
}

fn hash(input: &str) -> i32 {
    let mut hash_result = 0;

    for c in input.chars() {
        hash_result += c as i32;
        hash_result *= 17;
        hash_result %= 256;
    }

    hash_result
}

fn read_command(input: &str) -> Command {
    let input_split: Vec<&str> = input.split_terminator(&['=', '-'][..]).collect();

    let box_number = hash(input_split[0]) as usize;
    let lens_label = input_split[0];

    let command_type = match input_split.len() {
        2 => CommandType::Add(input_split[1].parse().unwrap()),
        _ => CommandType::Remove,
    };

    Command {
        box_number,
        lens_label,
        command_type,
    }
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let sequence = input_data.split(",");

    let mut boxes: Vec<Vec<(&str, i32)>> = vec![Vec::new(); 256];

    for cmd_str in sequence {
        let cmd = read_command(cmd_str);

        match cmd.command_type {
            CommandType::Remove => {
                boxes[cmd.box_number].retain(|&lens| lens.0 != cmd.lens_label);
            }
            CommandType::Add(focal_len) => {
                let mut lens_found = false;
                for lens in boxes[cmd.box_number].iter_mut() {
                    if lens.0 == cmd.lens_label {
                        lens.1 = focal_len;
                        lens_found = true;
                    }
                }
                if !lens_found {
                    boxes[cmd.box_number].push((cmd.lens_label, focal_len));
                }
            }
        }
    }

    let mut focusing_power_sum = 0;

    for box_i in 0..boxes.len() {
        for lens_i in 0..boxes[box_i].len() {
            focusing_power_sum += (1+box_i as i32) * (1+lens_i as i32) * boxes[box_i][lens_i].1;
        }
    }

    println!("{}", focusing_power_sum);

    Ok(())
}
