#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use counter::Counter;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut hands_bids = vec![];
    while let Some(line) = scan.read_line() {
        let sp = line.split_once(' ').unwrap();
        hands_bids.push((sp.0.to_owned(), sp.1.parse::<i32>().unwrap()));
    }

    let type_to_int = |h: &str| {
        let counts: Counter<_> = h.chars().collect();
        let counts = counts.most_common_ordered();
        match counts[0].1 {
            1 => 0, // high card
            2 => match counts[1].1 {
                1 => 1, // one pair
                _ => 2, // two pair
            },
            3 => match counts[1].1 {
                1 => 3, // three of a kind
                _ => 4, // full house
            },
            4 => 5, // four of a kind
            5 => 6, // five of a kind
            _ => unreachable!(),
        }
    };

    let strength_0 = "AKQJT98765432";

    let mut strength_bid_0: Vec<_> = hands_bids
        .iter()
        .map(|(h, b)| {
            let value = type_to_int(h) * (13i32.pow(5))
                + h.chars().fold(0, |acc, c| {
                    acc * 13 + (13 - strength_0.find(c).unwrap() as i32)
                });
            (value, b)
        })
        .collect();

    strength_bid_0.sort_unstable();
    let res_1 = strength_bid_0
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_, b))| acc + (i + 1) as i32 * b);

    let strength_1 = "AKQT98765432J";

    let mut strength_bid_1: Vec<_> = hands_bids
        .iter()
        .map(|(h, b)| {
            // update 'J's to match the most frequent non-'J'
            let counts: Counter<_> = h.chars().collect();
            let mut updated = h.clone();
            if counts[&'J'] != 0 {
                let top_2 = counts.k_most_common_ordered(2);
                if top_2.len() > 1 {
                    let mut tmp = [0; 1];
                    updated = updated.replace(
                        'J',
                        (if top_2[0].0 == 'J' {
                            top_2[1].0
                        } else {
                            top_2[0].0
                        })
                        .encode_utf8(&mut tmp),
                    )
                };
            }
            // get the value of the type with updated 'J', calculate value of
            // cards with 'J' as lowest value
            let value = type_to_int(&updated) * (13i32.pow(5))
                + h.chars().fold(0, |acc, c| {
                    acc * 13 + (13 - strength_1.find(c).unwrap() as i32)
                });
            (value, b)
        })
        .collect();

    strength_bid_1.sort_unstable();
    let res_2 = strength_bid_1
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_, b))| acc + (i + 1) as i32 * b);

    write!(out, "{}\n{}\n", res_1, res_2);
}
