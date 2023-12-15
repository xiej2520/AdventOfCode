#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use rustc_hash::FxHashMap;
use std::{
    io::{stdin, stdout, BufWriter, Write},
    iter::zip,
};

unsafe fn dfs(
    dp: &mut FxHashMap<(usize, usize), i64>,
    springs: *const u8,
    n: usize,
    groups: *const usize,
    m: usize,
    i: usize,
    j: usize,
) -> i64 {
    if let Some(res) = dp.get(&(i, j)) {
        return *res;
    }
    let block_size = *groups.add(j);
    if i + block_size > n {
        return 0;
    }

    for k in i..i + block_size {
        if *springs.add(k) == b'.' {
            return 0;
        }
    }

    if i + block_size < n && *springs.add(i + block_size) == b'#' {
        return 0;
    }

    if j == m - 1 {
        if i + block_size + 1 < n {
            for k in i + block_size + 1..n {
                if *springs.add(k) == b'#' {
                    return 0;
                }
            }
        }
        return 1;
    }

    let mut res = 0;
    for k in i + block_size + 1..n {
        if *springs.add(k) != b'.' {
            res += dfs(dp, springs, n, groups, m, k, j + 1);
        }
        if *springs.add(k) == b'#' {
            break;
        }
    }
    dp.insert((i, j), res);
    res
}
// 0.25s with iterate checks (no next)
// 0.25s with next precalculated
fn arrangements<'a>(springs: &'a [u8], groups: &'a [usize]) -> i64 {
    let mut res = 0;
    let mut dp = FxHashMap::default();
    for (i, &c) in springs.iter().enumerate() {
        if c != b'.' {
            unsafe {
                res += dfs(
                    &mut dp,
                    springs.as_ptr(),
                    springs.len(),
                    groups.as_ptr(),
                    groups.len(),
                    i,
                    0,
                );
            }
        }
        if c == b'#' {
            break;
        }
    }
    res
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let (mut springs, mut groups) = (vec![], vec![]);
    while let Some(line) = scan.read_line() {
        let split = line.split_once(' ').unwrap();
        springs.push(split.0.as_bytes().to_vec());
        groups.push(
            split
                .1
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let res_1: i64 = zip(&springs, &groups)
        .map(|(springs, groups)| arrangements(springs, groups))
        .sum();

    let res_2: i64 = zip(&springs, &groups)
        .map(|(springs, groups)| {
            let concat_springs = [
                &springs[..],
                &springs[..],
                &springs[..],
                &springs[..],
                &springs[..],
            ]
            .concat();
            let concat_groups = [
                &groups[..],
                &groups[..],
                &groups[..],
                &groups[..],
                &groups[..],
            ]
            .concat();
            arrangements(&concat_springs, &concat_groups)
        })
        .sum();

    write!(out, "{}\n{}\n", res_1, res_2);
}
