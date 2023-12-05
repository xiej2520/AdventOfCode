#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{UnsafeScanner, BIT};
use std::{
    error::Error,
    io::{stdin, stdout, BufWriter, Write},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut bit = BIT::new(200);

    let (mut res_1, mut res_2) = (0,0);
    let mut i = 1;
    while let Some(line) = scan.read_line() {
        let (_id, card) = line.split_once(':').ok_or("no split")?;
        let (winning, mine) = card.split_once('|').ok_or("no split")?;
        let winning: std::collections::HashSet<i32> = winning
            .split_ascii_whitespace()
            .filter_map(|w| w.parse::<i32>().ok())
            .collect();

        let matches = mine
            .split_ascii_whitespace()
            .filter(|w| w.parse::<i32>().map_or(false, |x| winning.contains(&x)))
            .count();
        
        res_1 += if matches == 0 { 0 } else { 1 << (matches - 1)};
        let cur_cards = bit.sum(i) + 1;
        bit.add(i as usize + 1, cur_cards);
        bit.add(i as usize + 1 + matches, -cur_cards);
        res_2 += cur_cards;
        i += 1;
    }

    write!(out, "{}\n{}\n", res_1, res_2);
    Ok(())
}
