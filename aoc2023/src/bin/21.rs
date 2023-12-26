#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{Grid, UnsafeScanner};
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
};

use Tile::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Plot,
            b'S' => Plot,
            b'#' => Rock,
            _ => panic!("Unexpected tile {}", value),
        }
    }
}
impl From<Tile> for u8 {
    fn from(value: Tile) -> Self {
        match value {
            Plot => b'.',
            Rock => b'#',
        }
    }
}

fn walk(grid: &Grid<Tile>, si: usize, sj: usize, max_steps: i32) -> Grid<i8> {
    let (m, n) = (grid.m(), grid.n());

    let mut parity = Grid::duplicate(-1, m, n);
    let mut q = VecDeque::new();
    q.push_back((si, sj));
    let mut steps = 1;
    parity[(si, sj)] = 0;
    while !q.is_empty() && steps <= max_steps {
        let sz = q.len();
        for _ in 0..sz {
            let (i, j) = q.pop_front().unwrap();
            let p = (steps & 1) as i8;
            if i > 0 && parity[(i - 1, j)] == -1 && grid[(i - 1, j)] == Plot {
                parity[(i - 1, j)] = p;
                q.push_back((i - 1, j));
            }
            if i + 1 < m && parity[(i + 1, j)] == -1 && grid[(i + 1, j)] == Plot {
                parity[(i + 1, j)] = p;
                q.push_back((i + 1, j));
            }
            if j > 0 && parity[(i, j - 1)] == -1 && grid[(i, j - 1)] == Plot {
                parity[(i, j - 1)] = p;
                q.push_back((i, j - 1));
            }
            if j + 1 < n && parity[(i, j + 1)] == -1 && grid[(i, j + 1)] == Plot {
                parity[(i, j + 1)] = p;
                q.push_back((i, j + 1));
            }
        }
        steps += 1;
    }
    parity
}

#[allow(dead_code)]
fn print_reachable(grid: &Grid<Tile>, parity: &Grid<i8>, steps: i32) {
    for (i, row) in parity.iter().enumerate() {
        for (j, &e) in row.iter().enumerate() {
            if grid[(i, j)] == Plot {
                if e == (steps & 1) as i8 {
                    print!("O");
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }
        println!();
    }
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let line = scan.read_line().unwrap().into_bytes();
    let mut grid = Grid::new(line.len());
    grid.push_row(line.iter().map(|&c| Tile::from(c)).collect());

    while let Some(line) = scan.read_line() {
        grid.push_row(line.as_bytes().iter().map(|&c| Tile::from(c)).collect());
    }
    let (m, n) = (grid.m(), grid.n());
    let (si, sj) = (m / 2, n / 2);

    // 130 steps to fill in all 4 corners, 131 steps to fill each corner
    // of next square with 1
    let parity = walk(&grid, si, sj, 64);
    let res_1: usize = parity
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&&e| e != -1 && (e & 1 == 64 & 1))
                .count()
        })
        .sum();

    // see https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    // The input data has an empty row and an empty column where the start is
    // The input data size is 131x131, 26501365 is 65 mod 131.
    // Taking 65 steps from the start reaches the sides of the input square
    // There is a gap between after the diamond of 65 steps from the start
    // So taking 26501365 steps means repeating the diamond and the 4 corners

    // This is also a quadratic function, used in the python solution

    let parity_65 = walk(&grid, si, sj, 65);
    let even_inner: usize = parity_65
        .iter()
        .map(|row| row.iter().filter(|&&e| e != -1 && (e & 1 == 0)).count())
        .sum();
    let odd_inner: usize = parity_65
        .iter()
        .map(|row| row.iter().filter(|&&e| e != -1 && (e & 1 == 1)).count())
        .sum();
    let parity_130 = walk(&grid, si, sj, 130);
    let even_corners = parity_130
        .iter()
        .map(|row| row.iter().filter(|&&e| e != -1 && (e & 1 == 0)).count())
        .sum::<usize>()
        - even_inner;
    let odd_corners = parity_130
        .iter()
        .map(|row| row.iter().filter(|&&e| e != -1 && (e & 1 == 1)).count())
        .sum::<usize>()
        - odd_inner;

    // number of squares we reach along the vertical and horizontal axes
    let k = (26501365 - (m / 2)) / m;
    let res_2 = ((k + 1) * (k + 1)) * (odd_corners + odd_inner)
        + (k * k) * (even_corners + even_inner)
        - (k + 1) * odd_corners
        + k * even_corners;

    write!(out, "{}\n{}\n", res_1, res_2);
}
