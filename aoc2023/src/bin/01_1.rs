#![allow(unused_must_use)]

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let d: Vec<&str> = "one two three four five six seven eight nine"
        .split(' ')
        .collect();

    let mut lines = vec![];
    while let Some(line) = scan.read_line() {
        lines.push(line);
    }
    writeln!(
        out,
        "{}",
        lines.iter().map(String::as_bytes).fold(0, |res, line| {
            res + line.iter().enumerate().fold(0, |acc, (i, c)| {
                let c = *c as usize - '0' as usize;
                if c > 0 && c <= 9 {
                    if acc == 0 {
                        11 * c
                    } else {
                        acc / 10 * 10 + c
                    }
                } else {
                    let mut f = acc;
                    for (index, num) in d.iter().enumerate() {
                        if i + num.len() <= line.len() && &line[i..i + num.len()] == num.as_bytes()
                        {
                            if acc == 0 {
                                f = 11 * (index + 1);
                            } else {
                                f = f / 10 * 10 + index + 1;
                            }
                        }
                    }
                    f
                }
            })
        })
    );
}
