#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, ops::RangeInclusive};

const ALLOWED_LOW_ITEMS: RangeInclusive<char> = 'a'..='z';
const ALLOWED_HIGH_ITEMS: RangeInclusive<char> = 'A'..='Z';

lazy_static! {
    static ref ITEM_PRIORITY: HashMap<char, u8> = {
        let mut m = HashMap::new();
        for (i, c) in ALLOWED_LOW_ITEMS.chain(ALLOWED_HIGH_ITEMS).enumerate() {
            m.insert(c, u8::try_from(i + 1).unwrap());
        }
        m
    };
}

fn get_priority(item: &char) -> u8 {
    ITEM_PRIORITY
        .get(item)
        .ok_or("Invalid item provided")
        .unwrap()
        .clone()
}

fn main() {
    let input = include_str!("../input.txt");

    let result = input
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|set| {
            set[0]
                .chars()
                .find(|c| set[1].contains(*c) && set[2].contains(*c))
                .unwrap()
        })
        .map(|c| get_priority(&c) as u32)
        .sum::<u32>();

    println!("Sum of priorities: {}", result);
}
