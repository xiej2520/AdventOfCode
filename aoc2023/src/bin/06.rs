#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::{
    io::{stdin, stdout, BufWriter, Write},
    iter::zip,
};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let line = scan.read_line().unwrap();
    let times: Vec<_> = line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect();
    let line = scan.read_line().unwrap();
    let dists: Vec<_> = line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect();

    fn ways(t: f64, d: f64) -> i32 {
        let sqrt_disc = f64::sqrt(t * t - 4.0 * d);
        (f64::floor((t + sqrt_disc) / 2.0 - 1e-5) - f64::ceil((t - sqrt_disc) / 2.0 + 1e-5)) as i32
            + 1
    }

    let res_1: i32 = zip(&times, &dists)
        .map(|(t, d)| {
            let t: f64 = t.parse().unwrap();
            let d: f64 = d.parse().unwrap();
            ways(t, d)
        })
        .product();

    let t: f64 = times.join("").parse().unwrap();
    let d: f64 = dists.join("").parse().unwrap();
    let res_2 = ways(t, d);
    write!(out, "{}\n{}\n", res_1, res_2);
}
