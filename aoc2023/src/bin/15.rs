#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

fn HASH(s: &[u8]) -> usize {
    s.iter().fold(0, |acc, &c| (acc + c as usize) * 17 & 0b11111111)
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let input = scan.read_line().unwrap().into_bytes();

    let mut res_1 = 0;
    let mut boxes: [Vec<(&[u8], usize)>; 256] = std::array::from_fn(|_| vec![]);
    for step in input.split(|&c| c == b',') {
        res_1 += HASH(step);
        if *step.last().unwrap() == b'-' {
            'find_box: for bx in &mut boxes {
                for (i, lens) in bx.iter().enumerate() {
                    if lens.0 == &step[0..step.len() - 1] {
                        bx.remove(i);
                        break 'find_box;
                    }
                }
            }
        } else {
            let mut iter = step.split(|&c| c == b'=');
            let label = iter.next().unwrap();
            let focal_length: usize = unsafe {
                std::str::from_utf8_unchecked(iter.next().unwrap())
                    .parse()
                    .unwrap()
            };

            let hash = HASH(label);
            let mut found = false;
            for bx in &mut boxes[hash] {
                if bx.0 == label {
                    bx.1 = focal_length;
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[hash].push((label, focal_length));
            }
        }
    }

    let res_2: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.iter()
                .enumerate()
                .map(|(j, (_, focal_length))| (1 + i) * (1 + j) * focal_length)
                .sum::<usize>()
        })
        .sum();

    write!(out, "{}\n{}\n", res_1, res_2);
}
