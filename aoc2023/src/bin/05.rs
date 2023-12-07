#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, BufWriter, Write},
};

// [a, b) -> [c, c + b - a)
#[derive(Debug, PartialEq, Eq)]
struct RangeMap {
    a: i64,
    b: i64,
    c: i64,
}
impl RangeMap {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }
    // unchecked
    fn map(&self, i: i64) -> i64 {
        self.c + i - self.a
    }
}
impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a)
    }
}

fn merge<T: Ord>(mut intervals: Vec<(T, T)>) -> Vec<(T, T)> {
    intervals.sort_unstable();
    let mut res: Vec<(T, T)> = vec![];
    for (l, r) in intervals {
        if !res.is_empty() {
            if l <= (*res.last().unwrap()).1 {
                if r > res.last().unwrap().1 {
                    res.last_mut().unwrap().1 = r;
                }
                // res[-1][1] = max(res[-1][1], r[1])
            } else {
                res.push((l, r));
            }
        } else {
            res.push((l, r));
        }
    }
    res
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let seeds: Vec<i64> = scan
        .read_line()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|w| w.parse::<i64>().ok())
        .collect();

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
                section.push(RangeMap::new(source, source + len, dest));
            }
        }
    }
    if !section.is_empty() {
        section.sort_unstable();
        almanac.push(section);
    }

    let res_1 = seeds
        .iter()
        .map(|&s| {
            let mut loc = s;
            for section in &almanac {
                let intersect = section.binary_search_by(|range| {
                    if range.b <= loc {
                        Ordering::Less
                    } else if range.a > loc {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                if let Ok(i) = intersect {
                    loc = section[i].map(loc);
                }
            }
            loc
        })
        .min()
        .unwrap();

    let locations = almanac.iter().fold(
        seeds
            .chunks(2)
            .map(|x| (x[0], x[0] + x[1]))
            .collect::<Vec<_>>(),
        |ranges, section| {
            merge(
                ranges
                    .iter()
                    .flat_map(|&(l, r)| {
                        let mut dest_intervals = vec![];
                        let mut l_window = l;
                        for range in section {
                            if l_window < range.a && range.a < r {
                                let r_window = min(r, range.a);
                                dest_intervals.push((l_window, r_window));
                                l_window = r_window;
                            }
                            if range.a <= l_window && l_window < range.b {
                                let r_window = min(r, range.b);
                                dest_intervals.push((range.map(l_window), range.map(r_window)));
                                l_window = r_window;
                            } else if r <= range.a {
                                break;
                            }
                        }
                        if l_window < r {
                            dest_intervals.push((l_window, r));
                        }
                        dest_intervals
                    })
                    .collect(),
            )
        },
    );
    let res_2 = locations.first().unwrap().0;
    write!(out, "{}\n{}\n", res_1, res_2);
}
