#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let d = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut res = 0;
    while let Some(line) = scan.read_line() {
        let line = line.as_bytes();
        let mut sum = 0;
        for (i, c) in line.iter().enumerate() {
            let c = *c as usize - '0' as usize;
            if c > 0 && c <= 9 {
                if sum == 0 {
                    sum = 11 * c;
                } else {
                    sum = sum / 10 * 10 + c;
                }
            } else {
                for (index, num) in d.iter().enumerate() {
                    if i + num.len() <= line.len() && &line[i..i + num.len()] == num.as_bytes() {
                        if sum == 0 {
                            sum = 11 * (index + 1);
                        } else {
                            sum = sum / 10 * 10 + index + 1;
                        }
                    }
                }
            }
        }
        res += sum;
    }
    writeln!(out, "{}", res);
}
