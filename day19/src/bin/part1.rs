use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs};

const FILENAME: &str = "input";

fn main() -> Result<()> {
    let input_string = fs::read_to_string(FILENAME)?;
    let input_data: Vec<&str> = input_string.split("\n\n").collect();
    let mut workflows = HashMap::new();

    for w in input_data[0].split_terminator("\n") {
        let pattern = Regex::new(r"(\w+)\{(.+?)\}")?;
        let captures = pattern.captures(w).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let rules_str = captures.get(2).unwrap().as_str();

        let rules: Vec<&str> = rules_str.split(",").collect();

        workflows.insert(name, rules);
    }

    let mut accepted_sum = 0;

    for part in input_data[1].split_terminator("\n") {
        let parameters: Vec<i32> = part[1..(part.len()) - 1]
            .split(",")
            .map(|p| p.split("=").nth(1).unwrap().parse().unwrap())
            .collect();

        let x = parameters[0];
        let m = parameters[1];
        let a = parameters[2];
        let s = parameters[3];

        let mut w_name = "in";

        while w_name != "R" && w_name != "A" {
            let w = workflows.get(w_name).unwrap();
            for &rule in w {
                let rule_spl: Vec<&str> = rule.split(":").collect();

                if rule_spl.len() == 2 {
                    let condition = rule_spl[0];
                    let name = rule_spl[1];

                    let parameter = match condition.chars().nth(0).unwrap() {
                        'x' => x,
                        'm' => m,
                        'a' => a,
                        's' => s,
                        _ => 0,
                    };

                    let number: i32 = condition[2..].parse().unwrap();

                    let rule_true = match condition.chars().nth(1).unwrap() {
                        '>' => parameter > number,
                        '<' => parameter < number,
                        _ => false,
                    };

                    if rule_true {
                        w_name = name;
                        break;
                    }
                } else {
                    w_name = rule;
                }
            }
        }

        if w_name == "A" {
            accepted_sum += x + m + a + s;
        }
    }

    println!("{}", accepted_sum);

    Ok(())
}
