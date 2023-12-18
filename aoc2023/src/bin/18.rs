#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::from_utf8_unchecked;

pub fn picks(points: &[(i64, i64)]) -> i64 {
    let mut res = 0;
    for i in 1..points.len() {
        let (px, py) = points[i - 1];
        let (qx, qy) = points[i];
        res += (px - qx) * (py + qy);
    }
    res += (points.last().unwrap().0 - points[0].0) * (points.last().unwrap().1 + points[0].1);
    res / 2
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let (mut points_1, mut points_2) = (vec![(0, 0)], vec![(0, 0)]);
    let (mut i1, mut j1, mut i2, mut j2) = (0, 0, 0, 0);

    let (mut res_1, mut res_2) = (0, 0);

    while let Some(line) = scan.read_line() {
        let bytes = line.as_bytes();
        let mut sp_iter = bytes.split(|&c| c == b' ');
        let dir_1 = *sp_iter.next().unwrap().first().unwrap();
        let dist_1: i64 = unsafe {
            from_utf8_unchecked(sp_iter.next().unwrap())
                .parse()
                .unwrap()
        };
        res_1 += dist_1; // side
        match dir_1 {
            b'R' => j1 += dist_1,
            b'L' => j1 -= dist_1,
            b'U' => i1 -= dist_1,
            b'D' => i1 += dist_1,
            _ => panic!("Unexpected direction '{}'", dir_1),
        }
        points_1.push((i1, j1));

        let color = sp_iter.next().unwrap();
        let dir_2 = color[color.len() - 2];
        let dist_2 = unsafe { i64::from_str_radix(from_utf8_unchecked(&color[2..7]), 16).unwrap() };
        res_2 += dist_2; // side
        match dir_2 {
            b'0' => j2 += dist_2, // 0 <=> R
            b'2' => j2 -= dist_2, // 2 <=> L
            b'3' => i2 -= dist_2, // 3 <=> U
            b'1' => i2 += dist_2, // 1 <=> D
            _ => panic!("Unexpected direction '{}'", dir_2),
        }
        points_2.push((i2, j2));
    }
    
    res_1 = res_1 / 2 + 1;
    res_1 -= picks(&points_1);
    res_2 = res_2 / 2 + 1;
    res_2 -= picks(&points_2);

    write!(out, "{}\n{}\n", res_1, res_2);
}
