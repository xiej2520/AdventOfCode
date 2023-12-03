#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let (r_max, g_max, b_max) = (12, 13, 14);

    let (mut res_1, mut res_2) = (0, 0);
    while let Some(line) = scan.read_line() {
        let (id, games) = line.split_once(": ").unwrap();
        let id: i32 = id[5..].parse().unwrap();
        let games = games.split("; ");
        let mut possible = true;
        let (mut r_min, mut g_min, mut b_min) = (0, 0, 0);
        for game in games {
            let items = game.split(", ");
            for item in items {
                let mut nc = item.split(' ');
                let n: i32 = nc.next().unwrap().parse().unwrap();
                let color = nc.next().unwrap();
                possible = match color {
                    "red" => {
                        r_min = max(r_min, n);
                        n <= r_max
                    }
                    "green" => {
                        g_min = max(g_min, n);
                        n <= g_max
                    }
                    "blue" => {
                        b_min = max(b_min, n);
                        n <= b_max
                    }
                    _ => panic!(),
                } && possible; // no short-circuit
            }
        }
        if possible {
            res_1 += id;
        }
        if r_min > 0 || g_min > 0 || b_min > 0 {
            res_2 += max(r_min, 1) * max(g_min, 1) * max(b_min, 1);
        }
    }
    writeln!(out, "{}", res_1);
    writeln!(out, "{}", res_2);
}
