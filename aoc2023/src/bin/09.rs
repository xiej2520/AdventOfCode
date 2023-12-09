#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let (mut res_1, mut res_2) = (0, 0);
    while let Some(line) = scan.read_line() {
        let mut history: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        fn sum_extrapolate(history: &[i64]) -> i64 {
            let mut diffs = vec![history.to_owned()];
            let mut all_zero = false;
            while !all_zero {
                all_zero = true;
                let mut diff = vec![];
                diffs.last().unwrap().windows(2).for_each(|w| {
                    diff.push(w[1] - w[0]);
                    all_zero = all_zero && (w[1] - w[0]) == 0;
                });
                diffs.push(diff);
            }
            diffs.iter().map(|s| s.last().unwrap()).sum()
        }
        res_1 += sum_extrapolate(&history);
        history.reverse();
        // could also do difference of first elements
        res_2 += sum_extrapolate(&history);
    }

    write!(out, "{}\n{}\n", res_1, res_2);
}
