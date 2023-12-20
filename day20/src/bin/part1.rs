use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use anyhow::Result;

const FILENAME: &str = "input";

#[derive(Debug)]
enum Module<'a> {
    Conj(HashMap<&'a str, bool>),
    Flip(bool),
    Broadcast,
}

fn send_pulse<'a>(conn: &mut HashMap<&str, (Module<'a>, Vec<&'a str>)>) -> (i64, i64) {
    let mut kłełe = VecDeque::new();
    kłełe.push_back((false, "broadcaster", "button"));

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    while !kłełe.is_empty() {
        let (pulse, mod_name, input_name) = kłełe.pop_front().unwrap();

        match pulse {
            true => {
                high_pulses += 1;
            }
            false => {
                low_pulses += 1;
            }
        }

        if !conn.contains_key(mod_name) {
            continue;
        }

        let module = conn.get_mut(mod_name).unwrap();

        match &mut module.0 {
            Module::Conj(recent_pulses) => {
                recent_pulses.insert(input_name, pulse);
                let mut all_high = true;
                for (_, p) in recent_pulses {
                    all_high &= *p;
                }
                for m in &module.1 {
                    kłełe.push_back((!all_high, *m, mod_name));
                }
            }
            Module::Flip(state) => {
                if !pulse {
                    *state ^= true;
                    for m in &module.1 {
                        kłełe.push_back((*state, *m, mod_name));
                    }
                }
            }
            Module::Broadcast => {
                for m in &module.1 {
                    kłełe.push_back((pulse, *m, mod_name));
                }
            }
        }
    }

    (high_pulses, low_pulses)
}

fn init_conjunctions<'a>(conn: &mut HashMap<&'a str, (Module<'a>, Vec<&'a str>)>) {
    let module_names: Vec<&str> = conn.keys().map(|&k| k).collect();
    for mod_name in module_names {
        let outputs = conn.get(mod_name).unwrap().1.clone();
        for o in outputs {
            if !conn.contains_key(o) {
                continue;
            }
            if let Module::Conj(recent_pulses) = &mut conn.get_mut(o).unwrap().0 {
                recent_pulses.insert(mod_name, false);
            }
        }
    }
}

fn main() -> Result<()> {
    let input_data = fs::read_to_string(FILENAME)?;
    let mut connections: HashMap<&str, (Module, Vec<&str>)> = input_data
        .split_terminator("\n")
        .map(|l| {
            let l_spl: Vec<&str> = l.split(" -> ").collect();
            let connected_modules: Vec<&str> = l_spl[1].split(", ").collect();
            let module_type = match l_spl[0].chars().nth(0).unwrap() {
                '%' => Module::Flip(false),
                '&' => Module::Conj(HashMap::new()),
                _ => Module::Broadcast,
            };
            let name = match l_spl[0] {
                "broadcaster" => l_spl[0],
                _ => &l_spl[0][1..],
            };
            (name, (module_type, connected_modules))
        })
        .collect();

    init_conjunctions(&mut connections);

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _ in 0..1000 {
        let (h, l) = send_pulse(&mut connections);
        high_pulses += h;
        low_pulses += l;
    }
    
    dbg!(high_pulses, low_pulses);
    let pulses = high_pulses * low_pulses;

    println!("{}", pulses);

    Ok(())
}
