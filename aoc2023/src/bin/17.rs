#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use rustc_hash::{FxHashMap, FxHasher};
use std::{
    collections::{VecDeque, BinaryHeap},
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
    pq.push((-((grid[0][1] - b'0') as i32), 0, 1, Right, 2));
    pq.push((-((grid[1][0] - b'0') as i32), 1, 0, Down, 2));
    
    while !pq.is_empty() {
        let (cost, i, j, d, rem) = pq.pop().unwrap();
        if let Some(_) = dp.get(&(i, j, d, rem)) {
            continue;
        }
        println!("{}, {} {} {:?} {}",cost, i,j,d,rem);
        dp.insert((i, j, d, rem), cost);

        match d {
            Left => {
                if j > 0 && rem > 0 {
                    pq.push((cost - (grid[i][j-1] - b'0') as i32, i, j-1, Left, rem - 1));
                }
                if i > 0 {
                    pq.push((cost - (grid[i-1][j] - b'0') as i32, i-1, j, Up, 2));
                }
                if i < m - 1 {
                    pq.push((cost - (grid[i+1][j] - b'0') as i32, i+1, j, Down, 2));
                }
            }
            Right => {
                if j < n - 1 && rem > 0 {
                    pq.push((cost - (grid[i][j+1] - b'0') as i32, i, j+1, Right, rem - 1));
                }
                if i > 0 {
                    pq.push((cost - (grid[i-1][j] - b'0') as i32, i-1, j, Up, 2));
                }
                if i < m - 1 {
                    pq.push((cost - (grid[i+1][j] - b'0') as i32, i+1, j, Down, 2));
                }
            }
            Up => {
                if i > 0 && rem > 0 {
                    pq.push((cost - (grid[i-1][j] - b'0') as i32, i-1, j, Up, rem - 1));
                }
                if j > 0 {
                    pq.push((cost - (grid[i][j-1] - b'0') as i32, i, j-1, Left, 2));
                }
                if j < n - 1 {
                    pq.push((cost - (grid[i][j+1] - b'0') as i32, i, j+1, Right, 2));
                }
            }
            Down => {
                if i < m - 1 && rem > 0 {
                    pq.push((cost - (grid[i+1][j] - b'0') as i32, i+1, j, Down, rem - 1));
                }
                if j > 0 {
                    pq.push((cost - (grid[i][j-1] - b'0') as i32, i, j-1, Left, 2));
                }
                if j < n - 1 {
                    pq.push((cost - (grid[i][j+1] - b'0') as i32, i, j+1, Right, 2));
                }
            }
        }

    }

    let mut res_1 = i32::MAX;
    for rem in 0..=2 {
        if let Some(r) = dp.get(&(m-1, n-1, Left, rem)) {
            res_1 = min(res_1, -r);
        }
        if let Some(r) = dp.get(&(m-1, n-1, Right, rem)) {
            res_1 = min(res_1, -r);
        }
        if let Some(r) = dp.get(&(m-1, n-1, Up, rem)) {
            res_1 = min(res_1, -r);
        }
        if let Some(r) = dp.get(&(m-1, n-1, Down, rem)) {
            res_1 = min(res_1, -r);
        }
    }
    let res_2 = 0;

    write!(out, "{}\n{}\n", res_1, res_2);
}
