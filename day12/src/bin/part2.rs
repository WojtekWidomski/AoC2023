use std::fs;

use anyhow::Result;

const FILENAME: &str = "test2.in";


fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let input_lines = input_data
        .split("\n")
        .filter(|&l| l != "");

    let mut combination_sum = 0;
    let mut unfolded_combination_sum = 0;


    for l in input_lines {
        let l_split: Vec<&str> = l.split(" ").collect();
        let row: Vec<char> = l_split[0].chars().collect();
        let groups: Vec<i32> = l_split[1].split(",").map(|x| x.parse().unwrap()).collect();

        
    }

    println!("{}", combination_sum);
    println!("unfolded {}", unfolded_combination_sum);

    Ok(())
}
