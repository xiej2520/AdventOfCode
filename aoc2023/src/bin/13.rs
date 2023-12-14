#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};


// return Some(index of perfect reflection), Some(index of one-imperfect reflection)
fn solve(nums: &[u32], width: u32) -> (Option<u32>, Option<u32>) {
    let mut res = (None, None);
    for i in 1..width {
        let mut diffs = 0;
        for &j in nums {
            // shift left to clear out unmatched bits, then shift right to pos
            let a = j << (32 - 2 * i) >> (32 - i);
            // reverse, then shift to get matched, then and to clear out unmatched
            let b = (j.reverse_bits() >> (32 - i)) & ((1 << (width - i)) - 1);
            diffs += (a ^ b).count_ones();
        }
        match diffs {
            0 => res.0 = Some(width - i),
            1 => res.1 = Some(width - i),
            _ => {},
        }
    }
    
    res
}
pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());
    
    let mut rows;
    let mut cols;
    let mut res_1 = 0;
    let mut res_2 = 0;
    while let Some(mut line) = scan.read_line() {
        rows = vec![];
        cols = vec![0; line.len()];
        while !line.trim().is_empty() {
            let mut cur_row = 0;
            for (&c, col) in line.as_bytes().iter().zip(&mut cols) {
                let b = (c == b'#') as u32;
                cur_row = (cur_row << 1) | b;
                *col = (*col << 1) | b;
            }
            rows.push(cur_row);
            line = match scan.read_line() {
                Some(line) => line,
                None => break,
            };
        }
        let v_res = solve(&rows, cols.len() as u32);
        let h_res = solve(&cols, rows.len() as u32);
        if let Some(i) = v_res.0 {
            res_1 += i;
        }
        if let Some(j) = h_res.0 {
            res_1 += 100 * j
        }
        if let Some(i) = v_res.1 {
            res_2 += i;
        }
        if let Some(j) = h_res.1 {
            res_2 += 100 * j
        }
    }

    write!(out, "{}\n{}\n", res_1, res_2);
}
