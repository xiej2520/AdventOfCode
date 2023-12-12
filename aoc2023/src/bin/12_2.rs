#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::{
    fmt::Display,
    io::{stdin, stdout, BufWriter, Write},
};
use rayon::prelude::*;

// 0.06s with bounds check
fn arrangements(springs: &[u8], groups: &[usize]) -> i64 {
    let mut dp = vec![0i64; springs.len()];

    // this doesn't work because we need to have everything to the left be not '.'
    // dp[i][j]: number of solutions with jth block placed at i or after
    // dp[i][j] = valid(i, j) * dp[i-1][j + block_size] + dp[i][j+1]

    // dp[i][j]: number of solutions with jth block placed at i
    // dp[i][j] = valid(i, j) * (dp[i-1][j + k + block_size]) for k in i..
    let block_size = *groups.last().unwrap();
    for i in (0..springs.len() - block_size + 1).rev() {
        if i > 0 && springs[i - 1] == b'#' {
            continue;
        }
        if i < springs.len() - block_size && springs[i + block_size] == b'#' {
            continue;
        }
        if springs[i..i+block_size].iter().all(|&c| c != b'.') {
            if springs[i+block_size..].iter().any(|&c| c == b'#') {
                break;
            }
            dp[i] = 1;
        }
    }

    for &block_size in groups.iter().rev().skip(1) {
        let mut new_dp = vec![0; springs.len()];
        for i in (0..springs.len() - block_size - 1).rev() {
            if i > 0 && springs[i - 1] == b'#' {
                continue;
            }
            if i < springs.len() - block_size && springs[i + block_size] == b'#' {
                continue;
            }
            if springs[i..i+block_size].iter().all(|&c| c != b'.') {
                for j in i+block_size+1..springs.len() {
                    if springs[j - 1] == b'#' {
                        break;
                    }
                    new_dp[i] += dp[j];
                }
            }
        }
        dp = new_dp;
    }
    
    let mut res = 0;
    for (i, &val) in dp.iter().enumerate() {
        if i > 0 && springs[i-1] == b'#' {
            break;
        }
        res += val;
    }

    res
}

//pub fn run(input: &str) -> impl Display {
//    let (mut springs, mut groups) = (vec![], vec![]);
//    for line in input.split('\n') {
//        if let Some(split) = line.split_once(' ') {
//            springs.push(split.0.as_bytes().to_vec());
//            groups.push(
//                split
//                    .1
//                    .split(',')
//                    .map(|x| x.parse::<usize>().unwrap())
//                    .collect::<Vec<_>>(),
//            );
//        }
//    }
//
//    let res_2: i64 = springs.into_par_iter().zip(&groups)
//        .map(|(springs, groups)| {
//            let concat_springs = [
//                &springs[..],
//                &springs[..],
//                &springs[..],
//                &springs[..],
//                &springs[..],
//            ]
//            .concat();
//            let concat_groups = [
//                &groups[..],
//                &groups[..],
//                &groups[..],
//                &groups[..],
//                &groups[..],
//            ]
//            .concat();
//            arrangements(&concat_springs, &concat_groups)
//        })
//        .sum();
//
//    res_2
//}
//pub fn main() {
//
//}
//pub fn main() {
//    let mut input = String::default();
//    let mut scan = UnsafeScanner::new(stdin().lock());
//    while let Some(line) = scan.read_line() {
//        input += &line;
//        input.push('\n');
//    }
//    println!("{}", run(&input));
//}

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

    let res_1: i64 = (&springs).into_par_iter().zip(&groups)
        .map(|(springs, groups)| arrangements(&springs, &groups))
        .sum();

    let res_2: i64 = springs.into_par_iter().zip(&groups)
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
