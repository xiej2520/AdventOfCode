#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use std::io::Read;


// return Some(index of perfect reflection)
fn solve(nums: &[u32], width: u32) -> Option<u32> {
    'width: for i in 1..width {
        for &j in nums {
            // shift left to clear out unmatched bits, then shift right to pos
            let a = j << (32 - 2 * i) >> (32 - i);
            // reverse, then shift to get matched, then and to clear out unmatched
            let b = (j.reverse_bits() >> (32 - i)) & ((1 << (width - i)) - 1);
            if (a ^ b).count_ones() > 0 {
                continue 'width;
            }
        }
        return Some(width - i);
    }
    
    None
}

pub fn run(input: &[u8]) -> u32 {
    let mut res_1 = 0;
    let mut iter = input.split(|&b| b == b'\n');
    while let Some(mut line) = iter.next() {
        let mut rows = [0; 17];
        let mut cols = [0; 17];
        //rows = vec![];
        //cols = vec![0; line.len()];
        let mut i = 0;
        let col_len = line.len();
        while !line.is_empty() {
            let mut cur_row = 0;
            for (&c, col) in line.iter().zip(&mut cols) {
                let b = (c == b'#') as u32;
                cur_row = (cur_row << 1) | b;
                *col = (*col << 1) | b;
            }
            rows[i] = cur_row;
            i += 1;
            line = match iter.next() {
                Some(line) => line,
                None => break,
            };
        }
        if let Some(i) = solve(&rows, col_len as u32) {
            res_1 += i;
        }
        else {
            res_1 += 100 * solve(&cols, i as u32).unwrap();
        }

    }
    res_1
}

pub fn main() {
    
    //let mut input = vec![];
    //std::io::stdin().lock().read_to_end(&mut input);

    //println!("{}", run(&input));
}