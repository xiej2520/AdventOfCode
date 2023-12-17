#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use rustc_hash::FxHashMap;
use std::io::{stdin, stdout, BufWriter, Write};

#[allow(non_snake_case)]
fn HASH(s: &[u8]) -> usize {
    s.iter()
        .fold(0, |acc, &c| ((acc + c as usize) * 17) & 0b11111111)
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let input = scan.read_line().unwrap().into_bytes();

    let mut res_1 = 0;
    struct LinkedLens {
        prev: u16,
        next: u16,
        focal_length: u8,
    }

    // linked list boxes
    //let mut lens_list = Vec::with_capacity(1386);
    let mut lens_list = Vec::with_capacity(1400);
    for i in 0..256 {
        lens_list.push(LinkedLens {
            prev: i,
            next: i,
            focal_length: 0,
        });
    }
    let mut lens_map: FxHashMap<&[u8], usize> = FxHashMap::default();

    for step in input.split(|&c| c == b',') {
        res_1 += HASH(step);
        if *step.last().unwrap() == b'-' {
            if let Some(index) = lens_map.remove(&step[0..step.len() - 1]) {
                let (l, r) = (lens_list[index].prev, lens_list[index].next);
                lens_list[l as usize].next = r;
                lens_list[r as usize].prev = l;
            }
        } else {
            let mut iter = step.split(|&c| c == b'=');
            let label = iter.next().unwrap();
            let focal_length: u8 = unsafe {
                std::str::from_utf8_unchecked(iter.next().unwrap())
                    .parse()
                    .unwrap()
            };
            let hash = HASH(label);
            if let Some(&index) = lens_map.get(label) {
                lens_list[index].focal_length = focal_length;
            }
            else {
                let i = lens_list.len();
                lens_map.insert(label, i);
                lens_list.push(LinkedLens {
                    prev: lens_list[hash].prev,
                    next: hash as u16,
                    focal_length: focal_length as u8
                });
                let prev = lens_list[hash].prev as usize;
                lens_list[prev].next = i as u16;
                lens_list[hash].prev = i as u16;
            }
        }
    }
    // max size of a box is 6
    // max size of all boxes is 280
    // 1130 lenses in total
    //println!("len {}",lens_list.len());
    //println!("cap {}",lens_list.capacity());

    let mut res_2 = 0;
    for i in 0..256 {
        let mut cur = lens_list[i].next;
        let mut j = 1;
        while cur != i as u16 {
            res_2 += (1 + i) * j * lens_list[cur as usize].focal_length as usize;
            cur = lens_list[cur as usize].next;
            j += 1;
        }
    }

    write!(out, "{}\n{}\n", res_1, res_2);
}
