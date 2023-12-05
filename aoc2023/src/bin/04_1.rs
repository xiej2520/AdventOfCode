#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut lines = vec![];
    while let Some(line) = scan.read_line() {
        lines.push(line);
    }
    // lazy segment tree better?
    let mut st = aoc2023::SegTree::new(lines.len() + 1);
    writeln!(
        out,
        "{:?}",
        lines
            .iter()
            .enumerate()
            .fold((0, 0), |(total_matches, total_cards), (i, line)| {
                let (_id, card) = line.split_once(':').ok_or("no split").unwrap();
                let (winning, mine) = card.split_once('|').ok_or("no split").unwrap();
                let winning: std::collections::HashSet<i32> = winning
                    .split_ascii_whitespace()
                    .filter_map(|w| w.parse::<i32>().ok())
                    .collect();

                let matches = mine
                    .split_ascii_whitespace()
                    .filter(|w| w.parse::<i32>().map_or(false, |x| winning.contains(&x)))
                    .count();

                let cur_cards = st.query(0, i) + 1;
                st.add(i + 1, cur_cards);
                st.add(i + matches + 1, -cur_cards);

                (
                    total_matches + if matches == 0 { 0 } else { 1 << (matches - 1) },
                    total_cards + cur_cards,
                )
            })
    );
}
