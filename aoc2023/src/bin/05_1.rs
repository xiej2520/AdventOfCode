#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::{cmp::Ordering, io::stdin};

// [a, b) -> [c, c + b - a)
#[derive(Debug, PartialEq, Eq)]
struct RangeMap {
    a: i64,
    c: i64,
    d: i64,
}
impl RangeMap {
    fn new(a: i64, c: i64, d: i64) -> Self {
        Self { a, c, d }
    }
    // unchecked
    fn rev_map(&self, i: i64) -> i64 {
        self.a + i - self.c
    }
}
impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.c.cmp(&other.c)
    }
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    //let mut out = BufWriter::new(stdout().lock());

    let mut seeds: Vec<_> = scan
        .read_line()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|w| w.parse::<i64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1]))
        .collect();
    seeds.sort_unstable();

    scan.read_line(); // skip whitespace
    let mut almanac = vec![];
    let mut section = vec![];
    while let Some(_header) = scan.read_line() {
        while let Some(line) = scan.read_line() {
            if line.trim().is_empty() {
                section.sort_unstable();
                almanac.push(section);
                section = vec![];
                break;
            } else {
                let mut iter = line.split_ascii_whitespace();
                let dest = iter.next().unwrap().parse().unwrap();
                let source = iter.next().unwrap().parse().unwrap();
                let len: i64 = iter.next().unwrap().parse().unwrap();
                section.push(RangeMap::new(source, dest, dest + len));
            }
        }
    }
    if !section.is_empty() {
        section.sort_unstable();
        almanac.push(section);
    }
    let max_seed = seeds.iter().map(|r| r.1).max().unwrap();

    const T: i64 = 32;
    std::thread::scope(|s| {
        for t in 1..=T {
            let almanac = &almanac;
            let seeds = &seeds;
            s.spawn(move || {
                for n in (t..=max_seed).step_by(T as usize) {
                    let start = almanac.iter().rev().fold(n, |loc, section| {
                        let intersect = section.binary_search_by(|range| {
                            if range.d <= loc {
                                Ordering::Less
                            } else if range.c > loc {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            }
                        });
                        if let Ok(i) = intersect {
                            section[i].rev_map(loc)
                        } else {
                            loc
                        }
                    });

                    if let Ok(_index) = seeds.binary_search_by(|range| {
                        if range.1 <= start {
                            Ordering::Less
                        } else if range.0 > start {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }) {
                        println!("success {} {}", start, n);
                        break;
                    }
                }
            });
        }
    });
    // 3.6s with --release
}
