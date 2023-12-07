use std::{cmp::Ordering, collections::HashMap, fs};

const FILENAME: &str = "input.in";

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn str_to_hand(hand_str: &str, cards_map: &HashMap<char, usize>) -> Vec<i32> {
    let mut hand = Vec::new();

    for c in hand_str.chars() {
        hand.push(cards_map[&c] as i32);
    }

    hand
}

fn hand_type(hand: &Vec<i32>) -> i32 {
    let mut card_n: HashMap<i32, i32> = HashMap::new();

    let mut j_number = 0;
    let j_int = (CARDS.len() - 1) as i32;

    for c in hand {
        if *c == j_int {
            j_number += 1;
            continue;
        }
        *card_n.entry(*c).or_insert(0) += 1;
    }

    if j_number == 5 {
        return 7;
    }

    let mut max_card_number = 0;
    let mut max_card_type = -1;

    for (&card_type, &number) in card_n.iter() {
        if number > max_card_number {
            max_card_number = number;
            max_card_type = card_type;
        }
    }

    *card_n.entry(max_card_type).or_insert(0) += j_number;

    match card_n.len() {
        1 => 7,
        2 => {
            let max_n = card_n.into_values().max().unwrap();
            match max_n {
                4 => 6,
                3 => 5,
                _ => 0,
            }
        }
        3 => {
            let max_n = card_n.into_values().max().unwrap();
            match max_n {
                3 => 4,
                2 => 3,
                _ => 0,
            }
        }
        4 => 2,
        5 => 1,
        _ => 0,
    }
}

fn compare_hands(a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
    if hand_type(a) > hand_type(b) {
        return Ordering::Greater;
    }
    if hand_type(b) > hand_type(a) {
        return Ordering::Less;
    }

    for i in 0..a.len() {
        if a[i] > b[i] {
            return Ordering::Less;
        }
        if a[i] < b[i] {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn main() {
    let input_data = fs::read_to_string(FILENAME).unwrap();
    let data_lines = input_data.split("\n").filter(|&l| l != "");

    let mut cards_map: HashMap<char, usize> = HashMap::new();
    for i in 0..CARDS.len() {
        cards_map.insert(CARDS[i], i);
    }

    let mut hands: Vec<(Vec<i32>, i32)> = data_lines
        .map(|l| {
            let l_spl: Vec<&str> = l.split(" ").collect();
            (str_to_hand(l_spl[0], &cards_map), l_spl[1].parse().unwrap())
        })
        .collect();

    hands.sort_by(|a, b| compare_hands(&a.0, &b.0));

    let mut res_sum = 0;

    for i in 1..(hands.len() + 1) {
        res_sum += hands[i - 1].1 * (i as i32);
    }

    println!("{}", res_sum);
}
