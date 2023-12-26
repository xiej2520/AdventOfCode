#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{Grid, UnsafeScanner};
use rustc_hash::FxHashSet;
use std::{io::{stdin, stdout, BufWriter, Write}, collections::VecDeque};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut bricks = vec![];
    while let Some(line) = scan.read_line() {
        let mut sp = line.split('~');
        let mut sp_pt = sp.next().unwrap().split(',');
        let mut p1 = Coord {
            x: sp_pt.next().unwrap().parse().unwrap(),
            y: sp_pt.next().unwrap().parse().unwrap(),
            z: sp_pt.next().unwrap().parse().unwrap(),
        };
        let mut sp_pt = sp.next().unwrap().split(',');
        let mut p2 = Coord {
            x: sp_pt.next().unwrap().parse().unwrap(),
            y: sp_pt.next().unwrap().parse().unwrap(),
            z: sp_pt.next().unwrap().parse().unwrap(),
        };
        if p2.z < p1.z {
            std::mem::swap(&mut p1, &mut p2);
        }
        bricks.push((p1, p2));
    }

    bricks.sort_by(|b1, b2| b1.0.z.cmp(&b2.0.z));
    let (max_x, max_y) = bricks.iter().fold((i16::MIN, i16::MIN), |acc, brick| {
        (
            max(acc.0, max(brick.0.x, brick.1.x)),
            max(acc.1, max(brick.0.y, brick.1.y)),
        )
    });

    let mut floor = Grid::duplicate((0, -1), max_x as usize + 1, max_y as usize + 1);
    let n = bricks.len();

    let mut supporting = vec![FxHashSet::default(); n];
    let mut supported_by = vec![FxHashSet::default(); n];
    for (idx, brick) in bricks.iter().enumerate() {
        let mut below = 0;
        for i in brick.0.x..=brick.1.x {
            for j in brick.0.y..=brick.1.y {
                below = max(below, floor[(i as usize, j as usize)].0);
            }
        }
        let new_height = below + brick.1.z - brick.0.z + 1;
        for i in brick.0.x..=brick.1.x {
            for j in brick.0.y..=brick.1.y {
                let (h, below_idx) = floor[(i as usize, j as usize)];
                if below_idx != -1 && h == below {
                    supporting[below_idx as usize].insert(idx);
                    supported_by[idx].insert(below_idx);
                }
                floor[(i as usize, j as usize)] = (new_height, idx as i16);
            }
        }
    }
    let safe: FxHashSet<_> = supporting
        .iter()
        .enumerate()
        .filter(|(_, supporting)| supporting.iter().all(|&b| supported_by[b].len() > 1))
        .map(|(i, _)| i)
        .collect();
    let res_1 = safe.len();

    let res_2: i32 = (0..n)
        .map(|i| {
            if safe.contains(&i) {
                return 0;
            }
            let mut fallen = FxHashSet::default();
            let mut q = VecDeque::new();
            fallen.insert(i as i16);
            q.push_back(i as i16);
            while let Some(b) = q.pop_front() {
                for &j in &supporting[b as usize] {
                    if fallen.is_superset(&supported_by[j]) {
                        fallen.insert(j as i16);
                        q.push_back(j as i16);
                    }
                }
            }
            fallen.len() as i32 - 1
        })
        .sum();
    write!(out, "{}\n{}\n", res_1, res_2);
}
