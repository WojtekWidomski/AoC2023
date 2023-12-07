use std::{collections::HashMap, fs, cmp::Ordering};

const FILENAME: &str = "input.in";

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
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

    for c in hand {
        if card_n.contains_key(c) {
            card_n.insert(*c, card_n.get(c).unwrap() + 1);
        } else {
            card_n.insert(*c, 1);
        }
    }

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
                // _ => {dbg!(&card_n); return 0;},
                _ => 0
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

    // hands.iter().for_each(|x| {
    //     let t = hand_type(&x.0);
    //     dbg!(x, t);
    // });

    hands.sort_by(|a, b| compare_hands(&a.0, &b.0));
    // hands.reverse();

    let mut res_sum = 0;

    for i in 1..(hands.len()+1) {
        // dbg!(&hands[i-1], i);
        res_sum += hands[i-1].1*(i as i32);
    }

    println!("{}", res_sum);
}
