#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};
use regex::Regex;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let dirs = scan.read_line().unwrap();
    scan.read_line();

    const N: usize = 26 * 26 * 26;

    // N as next means fake node
    let mut l = [N; N];
    let mut r = [N; N];

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    while let Some(line) = scan.read_line() {
        let Some(caps) = re.captures(&line) else {
            panic!()
        };
        let i = (&caps[1])
            .chars()
            .fold(0, |acc, c| acc * 26 + c as usize - 'A' as usize);
        l[i] = (&caps[2])
            .chars()
            .fold(0, |acc, c| acc * 26 + c as usize - 'A' as usize);
        r[i] = (&caps[3])
            .chars()
            .fold(0, |acc, c| acc * 26 + c as usize - 'A' as usize);
    }
    
    let mut res_1 = 0;
    let mut cur = 0;
    while cur != N - 1 {
        if dirs.as_bytes()[res_1 % dirs.len()] == b'L' {
            cur = l[cur];
        }
        else {
            cur = r[cur];
        }
        res_1 += 1;
    }
    let mut res_2 = 1;
    for start in 0..N {
        if start % 26 == 0 && l[start] != N {
            let mut i = 0;
            let mut cur = start;
            while cur % 26 != 25 {
                if dirs.as_bytes()[i % dirs.len()] == b'L' {
                    cur = l[cur];
                }
                else {
                    cur = r[cur];
                }
                i += 1;
            }
            res_2 /= gcd(res_2, i);
            res_2 *= i;
        }
    }
    

    write!(out, "{}\n{}\n", res_1, res_2);
}
