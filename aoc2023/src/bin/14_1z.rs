#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use rustc_hash::FxHashMap;
use std::{fmt, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    Empty,
}
use Rock::*;

impl From<u8> for Rock {
    fn from(value: u8) -> Self {
        match value {
            b'O' => Round,
            b'#' => Cube,
            b'.' => Empty,
            _ => panic!("Unexpected rock char {}", value),
        }
    }
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Round => write!(f, "O"),
            Cube => write!(f, "#"),
            Empty => write!(f, "."),
        }
    }
}

fn tilt_north(platformz: &mut [Rock], m: u16, n: u16) {
    for j in 0..n {
        let mut i_empty = 0;
        let mut i_rock = 1;
        'i_empty_loop: while i_rock < m {
            while platformz[index_of((i_empty, j)) as usize] != Empty {
                i_empty += 1;
                if i_empty >= m {
                    break 'i_empty_loop;
                }
            }
            i_rock = max(i_empty + 1, i_rock);
            while i_rock < m {
                match platformz[index_of((i_rock, j)) as usize] {
                    Round => {
                        (
                            platformz[index_of((i_empty, j)) as usize],
                            platformz[index_of((i_rock, j)) as usize],
                        ) = (Round, Empty);
                        i_empty += 1;
                        i_rock += 1;
                        break;
                    }
                    Cube => {
                        (i_empty, i_rock) = (i_rock + 1, i_rock + 2);
                        break;
                    }
                    Empty => {
                        i_rock += 1;
                    }
                }
            }
        }
    }
}
fn tilt_south(platformz: &mut [Rock], m: u16, n: u16) {
    for j in 0..n {
        let mut i_empty = m - 1;
        let mut i_rock = m - 2;
        'i_empty_loop: loop {
            while platformz[index_of((i_empty, j)) as usize] != Empty {
                if i_empty <= 1 {
                    break 'i_empty_loop;
                }
                i_empty -= 1;
            }
            i_rock = min(i_empty - 1, i_rock);
            loop {
                match platformz[index_of((i_rock, j)) as usize] {
                    Round => {
                        (
                            platformz[index_of((i_empty, j)) as usize],
                            platformz[index_of((i_rock, j)) as usize],
                        ) = (Round, Empty);
                        if i_rock == 0 {
                            break 'i_empty_loop;
                        }
                        i_empty -= 1;
                        i_rock -= 1;
                        break;
                    }
                    Cube => {
                        if i_rock <= 1 {
                            break 'i_empty_loop;
                        }
                        (i_empty, i_rock) = (i_rock - 1, i_rock - 2);
                        break;
                    }
                    Empty => {
                        if i_rock == 0 {
                            break 'i_empty_loop;
                        }
                        i_rock -= 1;
                    }
                }
            }
        }
    }
}

fn tilt_west(platformz: &mut [Rock], m: u16, n: u16) {
    for i in 0..m {
        let mut j_empty = 0;
        let mut j_rock = 1;
        'j_empty_loop: while j_rock < m {
            while platformz[index_of((i, j_empty)) as usize] != Empty {
                j_empty += 1;
                if j_empty >= n {
                    break 'j_empty_loop;
                }
            }
            j_rock = max(j_empty + 1, j_rock);
            while j_rock < m {
                match platformz[index_of((i, j_rock)) as usize] {
                    Round => {
                        (
                            platformz[index_of((i, j_empty)) as usize],
                            platformz[index_of((i, j_rock)) as usize],
                        ) = (Round, Empty);
                        j_empty += 1;
                        j_rock += 1;
                        break;
                    }
                    Cube => {
                        (j_empty, j_rock) = (j_rock + 1, j_rock + 2);
                        break;
                    }
                    Empty => {
                        j_rock += 1;
                    }
                }
            }
        }
    }
}
fn tilt_east(platformz: &mut [Rock], m: u16, n: u16) {
    for i in 0..m {
        let mut j_empty = n - 1;
        let mut j_rock = n - 2;
        'j_empty_loop: loop {
            while platformz[index_of((i, j_empty)) as usize] != Empty {
                if j_empty <= 1 {
                    break 'j_empty_loop;
                }
                j_empty -= 1;
            }
            j_rock = min(j_empty - 1, j_rock);
            loop {
                match platformz[index_of((i, j_rock)) as usize] {
                    Round => {
                        (
                            platformz[index_of((i, j_empty)) as usize],
                            platformz[index_of((i, j_rock)) as usize],
                        ) = (Round, Empty);
                        if j_rock == 0 {
                            break 'j_empty_loop;
                        }
                        j_empty -= 1;
                        j_rock -= 1;
                        break;
                    }
                    Cube => {
                        if j_rock <= 1 {
                            break 'j_empty_loop;
                        }
                        (j_empty, j_rock) = (j_rock - 1, j_rock - 2);
                        break;
                    }
                    Empty => {
                        if j_rock == 0 {
                            break 'j_empty_loop;
                        }
                        j_rock -= 1;
                    }
                }
            }
        }
    }
}

fn calculate_load(platformz: &[Rock], m: u16, n: u16) -> i32 {
    let mut res: i32 = 0;
    for i in 0..m {
        let mut k = 0;
        for j in 0..n {
            if let Round = platformz[index_of((i, j)) as usize] {
                k += 1;
            }
        }
        res += k * (m as i32 - i as i32);
    }
    res
}

pub fn run(input: &[u8]) -> i32 {
    let iter = input.split(|&b| b == b'\n');
    let mut platform = vec![];
    for line in iter {
        platform.push(line.iter().map(|&c| Rock::from(c)).collect::<Vec<_>>());
    }
    if let Some(l) = platform.last() {
        if l.is_empty() {
            platform.pop();
        }
    }

    let m = platform.len() as u16;
    let n = platform[0].len() as u16;
    let zsize = unsafe { bmi2::index_of((m, n)) + 1 };
    let mut platform_z = vec![Empty; zsize as usize];
    for (i, row) in platform.iter().enumerate() {
        for (j, &e) in row.iter().enumerate() {
            platform_z[index_of((i as u16, j as u16)) as usize] = e;
        }
    }
    drop(platform);

    let mut i = 0;
    let mut cycles_remaining = 1000000000;
    let mut seen = FxHashMap::default();
    let mut found = false;
    while i < cycles_remaining {
        tilt_north(&mut platform_z, m, n);
        tilt_west(&mut platform_z, m, n);
        tilt_south(&mut platform_z, m, n);
        tilt_east(&mut platform_z, m, n);

        //for i in 0..M {
        //    for j in 0..N {
        //        print!("{}", platform_z[index_of((i, j)) as usize].to_string());
        //    }
        //    println!();
        //}
        if !found {
            let mut repr = Vec::with_capacity((m * n) as usize);
            let (mut c, mut k) = (0u64, 0);
            for &e in &platform_z {
                match e {
                    Round => {
                        c = c << 1 | 1;
                        k += 1;
                    }
                    Empty => {
                        c <<= 1;
                        k += 1;
                    }
                    _ => {}
                }
                if k & 0b111111 == 0 {
                    repr.push(c);
                    k = 0;
                    c = 0;
                }
            }
            repr.push(c);
            if let Some(prev) = seen.get(&repr) {
                let cycle_len = i - prev;
                let rem = cycles_remaining - i;
                let div = rem / cycle_len;
                cycles_remaining -= div * cycle_len;
                found = true;
            } else {
                seen.insert(repr, i);
            }
        }
        i += 1;
    }

    calculate_load(&platform_z, m, n)
}

pub fn main() {
    let mut input = vec![];
    std::io::stdin().lock().read_to_end(&mut input);

    println!("{}", run(&input));
}

#[inline]
pub fn index_of((x, y): (u16, u16)) -> u32 {
    let packed = (x as u64) | ((y as u64) << 32);

    let first = (packed | (packed << 8)) & 0x00FF00FF00FF00FF;
    let second = (first | (first << 4)) & 0x0F0F0F0F0F0F0F0F;
    let third = (second | (second << 2)) & 0x3333333333333333;
    let fourth = (third | (third << 1)) & 0x5555555555555555;

    let x = fourth;
    let y = fourth >> 31;
    (x | y) as u32
}

#[inline]
pub fn index_of_64((x, y): (u32, u32)) -> u64 {
    let packed = (x as u128) | ((y as u128) << 64);

    let first = (packed | (packed << 16)) & 0x0000FFFF0000FFFF0000FFFF0000FFFF;
    let second = (first | (first << 8)) & 0x00FF00FF00FF00FF00FF00FF00FF00FF;
    let third = (second | (second << 4)) & 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F;
    let fourth = (third | (third << 2)) & 0x33333333333333333333333333333333;
    let fifth = (fourth | (fourth << 1)) & 0x55555555555555555555555555555555;

    let x = fifth;
    let y = fifth >> 63;
    (x | y) as u64
}

#[inline]
pub fn coord_of(idx: u32) -> (u16, u16) {
    let wide_idx = idx as u64;
    let packed = (wide_idx & 0x55555555) | ((wide_idx & 0xAAAAAAAA) << 31);

    let first = (packed | (packed >> 1)) & 0x3333333333333333;
    let second = (first | (first >> 2)) & 0x0F0F0F0F0F0F0F0F;
    let third = (second | (second >> 4)) & 0x00FF00FF00FF00FF;
    let fourth = third | (third >> 8);

    let x = fourth as u16;
    let y = (fourth >> 32) as u16;
    (x, y)
}

#[inline]
pub fn coord_of_64(idx: u64) -> (u32, u32) {
    let wide_idx = idx as u128;
    let packed = (wide_idx & 0x5555555555555555) | ((wide_idx & 0xAAAAAAAAAAAAAAAA) << 63);

    let first = (packed | (packed >> 1)) & 0x33333333333333333333333333333333;
    let second = (first | (first >> 2)) & 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F;
    let third = (second | (second >> 4)) & 0x00FF00FF00FF00FF00FF00FF00FF00FF;
    let fourth = (third | (third >> 8)) & 0x0000FFFF0000FFFF0000FFFF0000FFFF;
    let fifth = fourth | (fourth >> 16);

    let x = fifth as u32;
    let y = (fifth >> 64) as u32;
    (x, y)
}

#[allow(clippy::missing_safety_doc)]
#[cfg(target_arch = "x86_64")]
pub mod bmi2 {
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn index_of((x, y): (u16, u16)) -> u32 {
        use core::arch::x86_64::_pdep_u32;

        let x = _pdep_u32(x as u32, 0x55555555);
        let y = _pdep_u32(y as u32, 0xAAAAAAAA);
        x | y
    }
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn index_of_64((x, y): (u32, u32)) -> u64 {
        use core::arch::x86_64::_pdep_u64;

        let x = _pdep_u64(x as u64, 0x5555555555555555);
        let y = _pdep_u64(y as u64, 0xAAAAAAAAAAAAAAAA);
        x | y
    }
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn coord_of(idx: u32) -> (u16, u16) {
        use core::arch::x86_64::_pext_u32;

        let x = _pext_u32(idx, 0x55555555);
        let y = _pext_u32(idx, 0xAAAAAAAA);
        (x as u16, y as u16)
    }
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn coord_of_64(idx: u64) -> (u32, u32) {
        use core::arch::x86_64::_pext_u64;

        let x = _pext_u64(idx, 0x5555555555555555);
        let y = _pext_u64(idx, 0xAAAAAAAAAAAAAAAA);
        (x as u32, y as u32)
    }
}
