#![allow(unused_must_use)]
use std::io::{stdout, BufRead, BufWriter, Write};

pub fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    write!(out, "{:?}\n\n", lines);
}
