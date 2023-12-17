#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

// finds sum of pairwise distances, after expanding (distance, expansion * hubble constant)
// a must be sorted (with a[i].0 + a[i].1 * hc comparator)
fn sum_dists(a: &[(usize, usize)], hc: usize) -> usize {
    a.iter()
        .map(|&(d, e)| d + e * hc)
        .enumerate()
        .fold((0, 0), |(acc, prefix), (i, val)| {
            (acc + val * i - prefix, prefix + val)
        })
        .0
}
pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut image = vec![];
    let mut empty_row = vec![];

    let line = scan.read_line().unwrap();
    let mut empty_col = vec![true; line.len()];
    for (i, c) in line.chars().enumerate() {
        empty_col[i] = empty_col[i] && c == '.';
    }
    empty_row.push(line.chars().all(|c| c == '.'));
    image.push(line.into_bytes());

    while let Some(line) = scan.read_line() {
        for (i, c) in line.chars().enumerate() {
            empty_col[i] = empty_col[i] && c == '.';
        }
        empty_row.push(line.chars().all(|c| c == '.'));
        image.push(line.into_bytes());
    }
    let (m, n) = (image.len(), image.first().unwrap().len());

    let mut empty_row_prefix = Vec::with_capacity(m);
    empty_row_prefix.push(empty_row[0] as usize);
    for &is_empty_row in empty_row.iter().skip(1) {
        empty_row_prefix.push(*empty_row_prefix.last().unwrap() + is_empty_row as usize);
    }

    let mut empty_col_prefix = Vec::with_capacity(n);
    empty_col_prefix.push(empty_col[0] as usize);
    for &is_empty_col in empty_col.iter().skip(1) {
        empty_col_prefix.push(*empty_col_prefix.last().unwrap() + is_empty_col as usize);
    }

    let mut gx = vec![];
    let mut gy = vec![];
    for (i, row) in image.iter().enumerate() {
        for (j, &e) in row.iter().enumerate() {
            if e == b'#' {
                gx.push((i, empty_row_prefix[i]));
                gy.push((j, empty_col_prefix[j]));
            }
        }
    }
    gy.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let res_1 = sum_dists(&gx, 1) + sum_dists(&gy, 1);
    let res_2 = sum_dists(&gx, 1000000 - 1) + sum_dists(&gy, 1000000 - 1);

    write!(out, "{}\n{}\n", res_1, res_2);
}
