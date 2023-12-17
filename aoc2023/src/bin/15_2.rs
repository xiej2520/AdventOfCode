#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use rustc_hash::FxHashMap;
use std::io::Read;

#[allow(non_snake_case)]
fn HASH(s: &[u8]) -> usize {
    s.iter()
        .fold(0, |acc, &c| ((acc + c as usize) * 17) & 0b11111111)
}

pub fn run(input: &[u8]) -> u32 {
    unsafe {
        //for test runner
        //let input = &input[..input.len() - 1];
        //let mut res_1 = 0;
        struct LinkedLens {
            prev: u16,
            next: u16,
            focal_length: u8,
        }

        let mut lens_list = Vec::with_capacity(1500);
        for i in 0..256 {
            lens_list.push(LinkedLens {
                prev: i,
                next: i,
                focal_length: 0,
            });
        }
        let mut lens_map: FxHashMap<&[u8], usize> = FxHashMap::default();

        for step in input.split(|&c| c == b',') {
            //res_1 += HASH(step);
            if *step.last().unwrap() == b'-' {
                if let Some(index) = lens_map.remove(&step[..step.len() - 1]) {
                    let LinkedLens {
                        prev: l,
                        next: r,
                        focal_length: _,
                    } = *(lens_list.get_unchecked(index));
                    lens_list.get_unchecked_mut(l as usize).next = r;
                    lens_list.get_unchecked_mut(r as usize).prev = l;
                }
            } else {
                let label = &step[..step.len() - 2];
                let focal_length = step.last().unwrap() - b'0';
                let hash = HASH(label);
                if let Some(&index) = lens_map.get(label) {
                    lens_list.get_unchecked_mut(index).focal_length = focal_length;
                } else {
                    let i = lens_list.len();
                    lens_map.insert(label, i);
                    lens_list.push(LinkedLens {
                        prev: lens_list[hash].prev,
                        next: hash as u16,
                        focal_length,
                    });
                    let prev = lens_list.get_unchecked(hash).prev as usize;
                    lens_list.get_unchecked_mut(prev).next = i as u16;
                    lens_list.get_unchecked_mut(hash).prev = i as u16;
                }
            }
        }

        let mut res_2 = 0;
        for i in 0..256 {
            let mut cur = lens_list[i].next;
            let mut j = 1;
            while cur != i as u16 {
                res_2 +=
                    (1 + i) * j * lens_list.get_unchecked(cur as usize).focal_length as usize;
                cur = lens_list.get_unchecked(cur as usize).next;
                j += 1;
            }
        }
        res_2 as u32
    }
}

// DO NOT CHANGE
pub fn main() {
    let mut input = vec![];
    std::io::stdin().lock().read_to_end(&mut input);

    println!("{}", run(&input));
}
