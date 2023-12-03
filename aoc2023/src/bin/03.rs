#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut grid = vec![];
    while let Some(line) = scan.read_line() {
        grid.push(line);
    }
    let (m, n) = (grid.len(), grid[0].len());

    let mut count = 0;
    let mut numgrid: Vec<Vec<(i32, i32)>> = vec![vec![(-1, -1); n]; m];
    for (i, line) in grid.iter_mut().enumerate() {
        unsafe {
            let line = line.as_bytes_mut();
            let mut j = 0;
            while j < n {
                if line.get_unchecked(j).is_ascii_digit() {
                    let l = j;
                    j += 1;
                    while j < n && line.get_unchecked(j).is_ascii_digit() {
                        j += 1;
                    }
                    let num: i32 = std::str::from_utf8(&line[l..j]).unwrap().parse().unwrap();
                    for k in l..j {
                        *numgrid.get_unchecked_mut(i).get_unchecked_mut(k) = (count as i32, num);
                    }
                    count += 1;
                }
                j += 1;
            }
        }
    }

    let (mut res_1, mut res_2) = (0, 0);
    let mut included = vec![false; count];
    const DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    unsafe {
        for i in 0..m {
            for j in 0..n {
                let c = *grid.get_unchecked(i).as_bytes().get_unchecked(j);
                if c != b'.' && !c.is_ascii_digit() {
                    let mut parts = vec![];
                    for dir in DIRS {
                        let (x, y) = (dir.0 + i as i32, dir.1 + j as i32);
                        if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                            continue;
                        }
                        let (id, num) =
                            *numgrid.get_unchecked(x as usize).get_unchecked(y as usize);
                        if id != -1 {
                            if !included[id as usize] {
                                included[id as usize] = true;
                                res_1 += num;
                            }
                            if !parts.contains(&(id, num)) {
                                parts.push((id, num));
                            }
                        }
                    }
                    if parts.len() == 2 {
                        res_2 += parts[0].1 * parts[1].1;
                    }
                }
            }
        }
    }

    writeln!(out, "{}", res_1);
    writeln!(out, "{}", res_2);
}
