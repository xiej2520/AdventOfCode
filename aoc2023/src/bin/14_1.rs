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

fn tilt_north(platform: &mut [Vec<Rock>]) {
    let (m, n) = (platform.len(), platform[0].len());
    for j in 0..n {
        let mut i_empty = 0;
        let mut i_rock = 1;
        'i_empty_loop: while i_rock < m {
            while platform[i_empty][j] != Empty {
                i_empty += 1;
                if i_empty >= m {
                    break 'i_empty_loop;
                }
            }
            i_rock = max(i_empty + 1, i_rock);
            while i_rock < m {
                match platform[i_rock][j] {
                    Round => {
                        (platform[i_empty][j], platform[i_rock][j]) = (Round, Empty);
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
fn tilt_south(platform: &mut [Vec<Rock>]) {
    let (m, n) = (platform.len(), platform[0].len());
    for j in 0..n {
        let mut i_empty = m - 1;
        let mut i_rock = m - 2;
        'i_empty_loop: loop {
            while platform[i_empty][j] != Empty {
                if i_empty <= 1 {
                    break 'i_empty_loop;
                }
                i_empty -= 1;
            }
            i_rock = min(i_empty - 1, i_rock);
            loop {
                match platform[i_rock][j] {
                    Round => {
                        (platform[i_empty][j], platform[i_rock][j]) = (Round, Empty);
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

fn tilt_west(platform: &mut [Vec<Rock>]) {
    let (m, n) = (platform.len(), platform[0].len());
    for row in platform {
        let mut j_empty = 0;
        let mut j_rock = 1;
        'j_empty_loop: while j_rock < m {
            while row[j_empty] != Empty {
                j_empty += 1;
                if j_empty >= n {
                    break 'j_empty_loop;
                }
            }
            j_rock = max(j_empty + 1, j_rock);
            while j_rock < m {
                match row[j_rock] {
                    Round => {
                        (row[j_empty], row[j_rock]) = (Round, Empty);
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
fn tilt_east(platform: &mut [Vec<Rock>]) {
    let n = platform[0].len();
    for row in platform {
        let mut j_empty = n - 1;
        let mut j_rock = n - 2;
        'j_empty_loop: loop {
            while row[j_empty] != Empty {
                if j_empty <= 1 {
                    break 'j_empty_loop;
                }
                j_empty -= 1;
            }
            j_rock = min(j_empty - 1, j_rock);
            loop {
                match row[j_rock] {
                    Round => {
                        (row[j_empty], row[j_rock]) = (Round, Empty);
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

fn calculate_load(platform: &[Vec<Rock>]) -> i32 {
    let m = platform.len();
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| (row.iter().filter(|&&r| r == Round).count() * (m - i)) as i32)
        .sum()
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

    let mut i = 0;
    let mut n = 1000000000;
    let mut seen = FxHashMap::default();
    let mut found = false;
    while i < n {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        if !found {
            let mut repr = Vec::with_capacity(platform.len() * platform[0].len());
            let (mut c, mut k) = (0u64, 0);
            for row in &platform {
                for &e in row {
                    c = c << 1
                        | match e {
                            Round => 1,
                            _ => 0,
                        };
                    k += 1;
                    if k & 0b111111 == 0 {
                        repr.push(c);
                        k = 0;
                        c = 0;
                    }
                }
            }
            repr.push(c);
            if let Some(prev) = seen.get(&repr) {
                let cycle_len = i - prev;
                let rem = n - i;
                let div = rem / cycle_len;
                n -= div * cycle_len;
                found = true;
            } else {
                seen.insert(repr, i);
            }
        }
        i += 1;
    }

    calculate_load(&platform)
}

pub fn main() {
    let mut input = vec![];
    std::io::stdin().lock().read_to_end(&mut input);

    println!("{}", run(&input));
}
