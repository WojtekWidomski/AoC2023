use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs};

const FILENAME: &str = "input";

const MAX_RATING: i32 = 4000;
const PARAMETERS_NAMES: [char; 4] = ['x', 'm', 'a', 's'];

#[derive(Debug, Clone, Copy)]
struct Condition {
    parameter: char,
    greater: bool, // true - >, false - <, there is no =
    number: i32,
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    workflow: &'a str,
}

// Parses string like "a<2006" and converts it to Condition.
fn parse_condition(input: &str) -> Condition {
    Condition {
        parameter: input.chars().nth(0).unwrap(),
        greater: input.chars().nth(1).unwrap() == '>',
        number: input[2..].parse().unwrap(),
    }
}

// Parses rule string e.g. "a<2006:qkq".
fn parse_rule(input: &str) -> Rule {
    if !input.contains(":") {
        return Rule {
            condition: None,
            workflow: input,
        };
    }
    let input_spl: Vec<&str> = input.split(":").collect();
    let condition_str = input_spl[0];
    let workflow = input_spl[1];
    let condition = parse_condition(condition_str);
    Rule {
        condition: Some(condition),
        workflow,
    }
}

// condition_map is HashMap, where key is parameter name and value is minimum
// and maximum value of parameter. (x: (a, b) is x >= a and x <= b)
fn limit_to_conditon(new_cond: Condition, conditions_map: &mut HashMap<char, (i32, i32)>) {
    let cond = conditions_map.get_mut(&new_cond.parameter).unwrap();
    match new_cond.greater {
        true => {
            cond.0 = cond.0.max(new_cond.number + 1);
        }
        false => {
            cond.1 = cond.1.min(new_cond.number - 1);
        }
    }
}

fn reverse_condition(cond: Condition) -> Condition {
    match cond.greater {
        true => Condition {
            greater: false,
            number: cond.number + 1,
            ..cond
        },
        false => Condition {
            greater: true,
            number: cond.number - 1,
            ..cond
        },
    }
}

fn cond_combinations(conditions_map: &HashMap<char, (i32, i32)>) -> i64 {
    let mut result = 1;

    for c in conditions_map.values() {
        if c.1 < c.0 {
            return 0;
        }

        result *= c.1 as i64 - c.0 as i64 + 1;
    }

    result
}

fn cmb_dfs(
    w_name: &str,
    mut conditions_map: HashMap<char, (i32, i32)>,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> i64 {
    if w_name == "R" {
        return 0;
    }

    if w_name == "A" {
        return cond_combinations(&conditions_map);
    }

    let mut cmb_sum = 0;

    let workflow_rules = workflows.get(w_name).unwrap();

    for rule in workflow_rules {
        let mut new_condition_map = conditions_map.clone();
        if let Some(cond) = &rule.condition {
            limit_to_conditon(*cond, &mut new_condition_map);
            limit_to_conditon(reverse_condition(*cond), &mut conditions_map);
        }
        cmb_sum += cmb_dfs(rule.workflow, new_condition_map, workflows);
    }

    cmb_sum
}

fn main() -> Result<()> {
    let input_string = fs::read_to_string(FILENAME)?;
    let input_data: Vec<&str> = input_string.split("\n\n").collect();
    let mut workflows = HashMap::new();

    for w in input_data[0].split_terminator("\n") {
        let pattern = Regex::new(r"(\w+)\{(.+?)\}")?;
        let captures = pattern.captures(w).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let rules_str = captures.get(2).unwrap().as_str();
        let rules: Vec<Rule> = rules_str.split(",").map(|r| parse_rule(r)).collect();
        workflows.insert(name, rules);
    }

    let mut conditions_map = HashMap::new();
    for name in PARAMETERS_NAMES {
        conditions_map.insert(name, (1, MAX_RATING));
    }

    let combinations = cmb_dfs("in", conditions_map, &workflows);

    println!("{}", combinations);

    Ok(())
}
