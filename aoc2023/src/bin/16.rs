#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Tile {
    Empty = b'.',
    FMirror = b'/',
    BMirror = b'\\',
    VSplit = b'|',
    HSplit = b'-',
}
use Tile::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
        grid.push(unsafe {
            std::mem::transmute::<Vec<u8>, Vec<Tile>>(line.trim_end().as_bytes().to_vec())
        });
    }

    let (m, n) = (grid.len(), grid.first().unwrap().len());

    let fire_beam = |start: (usize, usize, Dir)| {
        let mut energized: Vec<Vec<u8>> = vec![vec![0; n]; m];
        let mut q = VecDeque::from([start]);
        energized[start.0][start.1] |= start.2 as u8;

        while !q.is_empty() {
            let (i, j, d) = q.pop_front().unwrap();

            let mut make_move = |i: usize, j, d| match d {
                Left => {
                    if j == 0 || energized[i][j - 1] & (Left as u8) != 0 {
                        return;
                    }
                    q.push_back((i, j - 1, Left));
                    energized[i][j - 1] |= Left as u8;
                }
                Right => {
                    if j + 1 == n || energized[i][j + 1] & (Right as u8) != 0 {
                        return;
                    }
                    q.push_back((i, j + 1, Right));
                    energized[i][j + 1] |= Right as u8;
                }
                Up => {
                    if i == 0 || energized[i - 1][j] & (Up as u8) != 0 {
                        return;
                    }
                    q.push_back((i - 1, j, Up));
                    energized[i - 1][j] |= Up as u8;
                }
                Down => {
                    if i + 1 == m || energized[i + 1][j] & (Down as u8) != 0 {
                        return;
                    }
                    q.push_back((i + 1, j, Down));
                    energized[i + 1][j] |= Down as u8;
                }
            };

            match grid[i][j] {
                Empty => {
                    make_move(i, j, d);
                }
                FMirror => match d {
                    Left => make_move(i, j, Down),
                    Right => make_move(i, j, Up),
                    Up => make_move(i, j, Right),
                    Down => make_move(i, j, Left),
                },
                BMirror => match d {
                    Left => make_move(i, j, Up),
                    Right => make_move(i, j, Down),
                    Up => make_move(i, j, Left),
                    Down => make_move(i, j, Right),
                },
                VSplit => {
                    make_move(i, j, Up);
                    make_move(i, j, Down);
                }
                HSplit => {
                    make_move(i, j, Left);
                    make_move(i, j, Right);
                }
            }
        }

        energized
            .iter()
            .flat_map(|row| row.iter().filter(|&&x| x != 0))
            .count()
    };

    let res_1 = fire_beam((0, 0, Right));
    let mut res_2 = 0;
    for i in 0..m {
        res_2 = max(res_2, fire_beam((i, 0, Right)));
        res_2 = max(res_2, fire_beam((i, n - 1, Left)));
    }
    for j in 0..n {
        res_2 = max(res_2, fire_beam((0, j, Down)));
        res_2 = max(res_2, fire_beam((m - 1, j, Up)));
    }

    write!(out, "{}\n{}\n", res_1, res_2);
}
