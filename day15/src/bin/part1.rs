use std::fs;

use anyhow::Result;

const FILENAME: &str = "input";

fn hash(input: &str) -> i32 {
    let mut hash_result = 0;

    for c in input.chars() {
        hash_result += c as i32;
        hash_result *= 17;
        hash_result %= 256;
    }

    hash_result
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let sequence = input_data.split(",");

    let hash_sum: i32 = sequence.map(|s| hash(s)).sum();

    println!("{}", hash_sum);

    Ok(())
}
