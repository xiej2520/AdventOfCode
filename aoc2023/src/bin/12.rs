#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use cached::proc_macro::cached;
//use memoize::memoize;
use std::{
    io::{stdin, stdout, BufWriter, Write},
    iter::zip, sync::Arc,
};

// 1.06s with cached + Arc, 1.28s with memoize + Rc
#[cached]
fn dfs(springs: Arc<[u8]>, groups: Arc<[usize]>, i: usize, j: usize) -> i64 {
    let n = springs.len();
    if i + groups[j] > n {
        return 0;
    }

    for &s in &springs[i..i + groups[j]] {
        if s == b'.' {
            return 0;
        }
    }

    if i + groups[j] < n && springs[i + groups[j]] == b'#' {
        return 0;
    }

    if j == groups.len() - 1 {
        if i + groups[j] + 1 < n {
            for &s in &springs[i + groups[j] + 1..] {
                if s == b'#' {
                    return 0;
                }
            }
        }
        return 1;
    }

    let mut res = 0;
    for k in i + groups[j] + 1..n {
        if springs[k] != b'.' {
            res += dfs(springs.clone(), groups.clone(), k, j + 1);
        }
        if springs[k] == b'#' {
            break;
        }
    }
    res
}

fn arrangements(springs: Arc<[u8]>, groups: Arc<[usize]>) -> i64 {
    let mut res = 0;
    for (i, &c) in springs.iter().enumerate() {
        if c != b'.' {
            res += dfs(springs.clone(), groups.clone(), i, 0);
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
        .map(|(springs, groups)| arrangements(springs.clone().into(), groups.clone().into()))
        .sum();

    let res_2: i64 = zip(springs, groups)
        .map(|(springs, groups)| {
            arrangements(
                [
                    &springs[..],
                    &springs[..],
                    &springs[..],
                    &springs[..],
                    &springs[..],
                ]
                .concat()
                .into(),
                [
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                ]
                .concat()
                .into(),
            )
        })
        .sum();

    write!(out, "{}\n{}\n", res_1, res_2);
}
