#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use rustc_hash::FxHashMap;
use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Write},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left = 0b0001,
    Right = 0b0010,
    Up = 0b0100,
    Down = 0b1000,
}
use Dir::*;

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut grid = vec![];
    while let Some(line) = scan.read_line() {
        grid.push(line.into_bytes());
    }
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = FxHashMap::default();

    let mut pq = BinaryHeap::new();
    pq.push((-((grid[0][1] - b'0') as i32), 0, 1, Right, 9));
    pq.push((-((grid[1][0] - b'0') as i32), 1, 0, Down, 9));

    while !pq.is_empty() {
        let (cost, i, j, d, rem) = pq.pop().unwrap();
        if dp.get(&(i, j, d, rem)).is_some() {
            continue;
        }
        //println!("{}, {} {} {:?} {}",cost, i,j,d,rem);
        dp.insert((i, j, d, rem), cost);

        match d {
            Left => {
                if j > 0 && rem > 0 {
                    pq.push((
                        cost - (grid[i][j - 1] - b'0') as i32,
                        i,
                        j - 1,
                        Left,
                        rem - 1,
                    ));
                }
                if i > 0 && rem < 7 {
                    pq.push((cost - (grid[i - 1][j] - b'0') as i32, i - 1, j, Up, 9));
                }
                if i < m - 1 && rem < 7 {
                    pq.push((cost - (grid[i + 1][j] - b'0') as i32, i + 1, j, Down, 9));
                }
            }
            Right => {
                if j < n - 1 && rem > 0 {
                    pq.push((
                        cost - (grid[i][j + 1] - b'0') as i32,
                        i,
                        j + 1,
                        Right,
                        rem - 1,
                    ));
                }
                if i > 0 && rem < 7 {
                    pq.push((cost - (grid[i - 1][j] - b'0') as i32, i - 1, j, Up, 9));
                }
                if i < m - 1 && rem < 7 {
                    pq.push((cost - (grid[i + 1][j] - b'0') as i32, i + 1, j, Down, 9));
                }
            }
            Up => {
                if i > 0 && rem > 0 {
                    pq.push((cost - (grid[i - 1][j] - b'0') as i32, i - 1, j, Up, rem - 1));
                }
                if j > 0 && rem < 7 {
                    pq.push((cost - (grid[i][j - 1] - b'0') as i32, i, j - 1, Left, 9));
                }
                if j < n - 1 && rem < 7 {
                    pq.push((cost - (grid[i][j + 1] - b'0') as i32, i, j + 1, Right, 9));
                }
            }
            Down => {
                if i < m - 1 && rem > 0 {
                    pq.push((
                        cost - (grid[i + 1][j] - b'0') as i32,
                        i + 1,
                        j,
                        Down,
                        rem - 1,
                    ));
                }
                if j > 0 && rem < 7 {
                    pq.push((cost - (grid[i][j - 1] - b'0') as i32, i, j - 1, Left, 9));
                }
                if j < n - 1 && rem < 7 {
                    pq.push((cost - (grid[i][j + 1] - b'0') as i32, i, j + 1, Right, 9));
                }
            }
        }
    }

    let mut res_2 = i32::MAX;
    for rem in 0..=6 {
        if let Some(r) = dp.get(&(m - 1, n - 1, Left, rem)) {
            res_2 = min(res_2, -r);
        }
        if let Some(r) = dp.get(&(m - 1, n - 1, Right, rem)) {
            res_2 = min(res_2, -r);
        }
        if let Some(r) = dp.get(&(m - 1, n - 1, Up, rem)) {
            res_2 = min(res_2, -r);
        }
        if let Some(r) = dp.get(&(m - 1, n - 1, Down, rem)) {
            res_2 = min(res_2, -r);
        }
    }
    let res_2 = 0;

    write!(out, "{}\n{}\n", res_2, res_2);
}
