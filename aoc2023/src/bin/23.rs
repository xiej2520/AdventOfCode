#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{Grid, UnsafeScanner};
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
};

fn junction_adj(mut grid: Grid<u8>) -> Vec<Vec<(usize, usize)>> {
    let (m, n) = (grid.m(), grid.n());
    let mut junctions = Grid::duplicate(-1, m, n);
    let mut junction_count = 2;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let mut splits = 0;
            if grid[(i, j)] == b'#' {
                continue;
            }
            if i > 0 && grid[(i - 1, j)] != b'#' {
                splits += 1;
            }
            if i + 1 < m && grid[(i + 1, j)] != b'#' {
                splits += 1;
            }
            if j > 0 && grid[(i, j - 1)] != b'#' {
                splits += 1;
            }
            if j + 1 < n && grid[(i, j + 1)] != b'#' {
                splits += 1;
            }
            if splits >= 3 {
                junctions[(i, j)] = junction_count as i32;
                junction_count += 1;
            }
        }
    }
    junctions[(0, 1)] = 0;
    junctions[(m - 1, n - 2)] = 1;
    let mut adj = vec![vec![]; junction_count];

    let mut q_junctions = VecDeque::new();
    q_junctions.push_back((0, 1, 0));
    while let Some((i, j, idx)) = q_junctions.pop_front() {
        grid[(i, j)] = b'#';
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if i as i32 + di < 0
                || i + di as usize >= m
                || j as i32 + dj < 0
                || j + dj as usize >= n
            {
                continue;
            }
            let (mut i, mut j) = (i + di as usize, j + dj as usize);
            let mut length = 1;
            while grid[(i, j)] != b'#' {
                if junctions[(i, j)] != -1 {
                    adj[idx].push((junctions[(i, j)] as usize, length));
                    adj[junctions[(i, j)] as usize].push((idx, length));
                    q_junctions.push_back((i, j, junctions[(i, j)] as usize));
                    break;
                }
                grid[(i, j)] = b'#';

                if i > 0 && grid[(i - 1, j)] != b'#' {
                    i -= 1;
                } else if i + 1 < m && grid[(i + 1, j)] != b'#' {
                    i += 1;
                } else if j > 0 && grid[(i, j - 1)] != b'#' {
                    j -= 1;
                } else if j + 1 < n && grid[(i, j + 1)] != b'#' {
                    j += 1;
                } else {
                    break;
                }
                length += 1;
            }
        }
    }
    adj
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let line = scan.read_line().unwrap().into_bytes();
    let mut grid = Grid::new(line.len());
    grid.push_row(line);

    while let Some(line) = scan.read_line() {
        grid.push_row(line.into_bytes());
    }
    let (m, n) = (grid.m(), grid.n());

    let mut visited = Grid::duplicate(false, m, n);
    visited[(0, 1)] = true;

    // basic dfs for longest path for part 1
    fn dfs(i: usize, j: usize, grid: &Grid<u8>, visited: &mut Grid<bool>) -> i32 {
        if i == grid.m() - 1 && j == grid.n() - 2 {
            return 0;
        }
        visited[(i, j)] = true;
        let mut res = -1;
        match grid[(i, j)] {
            b'^' => {
                if !visited[(i - 1, j)] {
                    res = 1 + dfs(i - 1, j, grid, visited);
                }
            }
            b'v' => {
                if !visited[(i + 1, j)] {
                    res = 1 + dfs(i + 1, j, grid, visited);
                }
            }
            b'>' => {
                if !visited[(i, j + 1)] {
                    res = 1 + dfs(i, j + 1, grid, visited);
                }
            }
            b'<' => {
                if !visited[(i, j - 1)] {
                    res = 1 + dfs(i, j - 1, grid, visited);
                }
            }
            b'.' => {
                if i > 0 && grid[(i - 1, j)] != b'#' && !visited[(i - 1, j)] {
                    res = max(res, 1 + dfs(i - 1, j, grid, visited));
                }
                if i + 1 < grid.m() && grid[(i + 1, j)] != b'#' && !visited[(i + 1, j)] {
                    res = max(res, 1 + dfs(i + 1, j, grid, visited));
                }
                if j > 0 && grid[(i, j - 1)] != b'#' && !visited[(i, j - 1)] {
                    res = max(res, 1 + dfs(i, j - 1, grid, visited));
                }
                if j + 1 < grid.n() && grid[(i, j + 1)] != b'#' && !visited[(i, j + 1)] {
                    res = max(res, 1 + dfs(i, j + 1, grid, visited));
                }
            }
            _ => unreachable!(),
        }
        visited[(i, j)] = false;
        res
    }

    let res_1 = dfs(0, 1, &grid, &mut visited);

    // create adjacency list for junctions from grid
    let adj = junction_adj(grid.clone());
    // then another basic dfs for part 2
    fn dfs2(i: usize, visited: usize, adj: &[Vec<(usize, usize)>]) -> usize {
        if i == 1 {
            0
        } else {
            let mut res = 0;
            for &(j, dist) in &adj[i] {
                if (visited & 1 << j) == 0 {
                    res = max(res, dist + dfs2(j, visited | 1 << j, adj));
                }
            }
            res
        }
    }

    let res_2 = dfs2(0, 1 << 0, &adj);

    write!(out, "{}\n{}\n", res_1, res_2);
}
