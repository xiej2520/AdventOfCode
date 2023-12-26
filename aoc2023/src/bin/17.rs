#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{UnsafeScanner, Grid};
use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Write},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}
use Dir::*;

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut grid = vec![];
    while let Some(line) = scan.read_line() {
        grid.push(line.into_bytes().iter().map(|&c| c - b'0').collect::<Vec<_>>());
    }
    let (m, n) = (grid.len(), grid[0].len());
    
    // grid[i][j][dir][rem]
    let mut visited = Grid::duplicate([[i16::MIN; 3]; 4], m, n);

    let mut pq = BinaryHeap::new();
    pq.push((-(grid[0][1] as i16), 0, 1, Right, 2));
    pq.push((-(grid[1][0] as i16), 1, 0, Down, 2));

    while !pq.is_empty() {
        let (cost, i, j, d, rem) = pq.pop().unwrap();
        if visited[(i, j)][d as usize][rem] > i16::MIN {
            continue;
        }
        visited[(i, j)][d as usize][rem] = cost;
        if i == m-1 && j == n-1 {
            break;
        }

        let mut try_move = |dir, i: usize, j, rem| {
            match dir {
                Left => {
                    if j > 0 {
                        pq.push((cost - grid[i][j-1] as i16, i, j-1, Left, rem));
                    }
                },
                Right => {
                    if j < n - 1 {
                        pq.push((cost - grid[i][j+1] as i16, i, j+1, Right, rem));
                    }
                },
                Up => {
                    if i > 0 {
                        pq.push((cost - grid[i-1][j] as i16, i-1, j, Up, rem));
                    }
                },
                Down => {
                    if i < m - 1 {
                        pq.push((cost - grid[i+1][j] as i16, i+1, j, Down, rem));
                    }
                }
            }
        };

        match d {
            Left => {
                if rem > 0 {
                    try_move(Left, i, j, rem - 1);
                }
                try_move(Up, i, j, 2);
                try_move(Down, i, j, 2);
            }
            Right => {
                if rem > 0 {
                    try_move(Right, i, j, rem - 1);
                }
                try_move(Up, i, j, 2);
                try_move(Down, i, j, 2);
            }
            Up => {
                if rem > 0 {
                    try_move(Up, i, j, rem - 1);
                }
                try_move(Left, i, j, 2);
                try_move(Right, i, j, 2);
            }
            Down => {
                if rem > 0 {
                    try_move(Down, i, j, rem - 1);
                }
                try_move(Left, i, j, 2);
                try_move(Right, i, j, 2);
            }
        }
    }

    let mut res_1 = i16::MIN;
    for dir in 0..4 {
        for rem in 0..3 {
            res_1 = max(res_1, visited[(m-1, n-1)][dir][rem]);
        }
    }
    res_1 = -res_1;

    let mut visited = Grid::duplicate([[i16::MIN; 10]; 4], m, n);

    let mut pq = BinaryHeap::new();
    pq.push((-(grid[0][1] as i16), 0, 1, Right, 9));
    pq.push((-(grid[1][0] as i16), 1, 0, Down, 9));

    while !pq.is_empty() {
        let (cost, i, j, d, rem) = pq.pop().unwrap();
        if visited[(i, j)][d as usize][rem] > i16::MIN {
            continue;
        }
        visited[(i, j)][d as usize][rem] = cost;
        if i == m-1 && j == n-1  && rem <= 6{
            break;
        }

        let mut try_move = |dir, i: usize, j, rem| {
            match dir {
                Left => {
                    if j > 0 {
                        pq.push((cost - grid[i][j-1] as i16, i, j-1, Left, rem));
                    }
                },
                Right => {
                    if j < n - 1 {
                        pq.push((cost - grid[i][j+1] as i16, i, j+1, Right, rem));
                    }
                },
                Up => {
                    if i > 0 {
                        pq.push((cost - grid[i-1][j] as i16, i-1, j, Up, rem));
                    }
                },
                Down => {
                    if i < m - 1 {
                        pq.push((cost - grid[i+1][j] as i16, i+1, j, Down, rem));
                    }
                }
            }
        };

        match d {
            Left => {
                if rem > 0 {
                    try_move(Left, i, j, rem - 1);
                }
                // 4 steps taken at 6 and below
                if rem <= 6 {
                    try_move(Up, i, j, 9);
                    try_move(Down, i, j, 9);
                }
            }
            Right => {
                if rem > 0 {
                    try_move(Right, i, j, rem - 1);
                }
                if rem <= 6 {
                    try_move(Up, i, j, 9);
                    try_move(Down, i, j, 9);
                }
            }
            Up => {
                if rem > 0 {
                    try_move(Up, i, j, rem - 1);
                }
                if rem <= 6 {
                    try_move(Left, i, j, 9);
                    try_move(Right, i, j, 9);
                }
            }
            Down => {
                if rem > 0 {
                    try_move(Down, i, j, rem - 1);
                }
                if rem <= 6 {
                    try_move(Left, i, j, 9);
                    try_move(Right, i, j, 9);
                }
            }
        }
    }

    let mut res_2 = i16::MIN;
    for dir in 0..4 {
        for rem in 0..=6 {
            res_2 = max(res_2, visited[(m-1, n-1)][dir][rem]);
        }
    }
    res_2 = -res_2;


    write!(out, "{}\n{}\n", res_1, res_2);
}
