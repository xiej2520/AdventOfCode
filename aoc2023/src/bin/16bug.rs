#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use std::{collections::VecDeque, io::BufRead};

enum Tile {
    Empty,
    FMirror,
    BMirror,
    VSplit,
    HSplit,
}

enum Dir {
    Left = 0b0001,
    Right = 0b0010,
    Up = 0b0100,
    Down = 0b1000,
}
use Dir::*;

pub fn main() {
    let mut grid: Vec<Vec<Tile>> = vec![];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        grid.push(unsafe { std::mem::transmute::<Vec<u8>, Vec<Tile>>(line.trim_end().as_bytes().to_vec()) });
    }

    let (m, n) = (grid.len(), grid.first().unwrap().len());

    let fire_beam = |start: (usize, usize, Dir)| {
        let mut energized: Vec<Vec<u8>> = vec![vec![0; n]; m];
        let mut q = VecDeque::from([start]);

        while !q.is_empty() {
            // only works with type annotation on i, j here:
            let mut make_move = |i, j, d| match d {
                Left => {
                    if j == 0 || energized[i][j - 1] | (Left as u8) != 0 {
                        return;
                    }
                }
                Right => {
                    if j + 1 == n || energized[i][j + 1] | (Right as u8) != 0 {
                        return;
                    }
                    q.push_back((i, j + 1, Right));
                    energized[i][j + 1] |= Right as u8;
                }
                _ => {}
            };
        }

        0
    };

    let res_1 = fire_beam((0, 0, Right));
    println!("{}", res_1);
}
