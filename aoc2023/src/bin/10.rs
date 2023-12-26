#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
use Tile::*;

impl Tile {
    #[allow(dead_code)]
    fn is_pipe(self) -> bool {
        matches!(self, NS | EW | NE | NW | SW | SE)
    }

    // dir going into tile
    fn accepts(self, dir: Dir) -> bool {
        match self {
            NS => dir == N || dir == S,
            EW => dir == E || dir == W,
            NE => dir == N || dir == E,
            NW => dir == N || dir == W,
            SW => dir == S || dir == W,
            SE => dir == S || dir == E,
            _ => false,
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => panic!("Invalid char ({}) to tile", c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TileStatus {
    Enclosed,
    Loop,
    Outside,
}
use TileStatus::*;

impl std::fmt::Display for TileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Enclosed => "`",
                Loop => "#",
                Outside => " ",
            }
        )
    }
}

// determines what the start pipe has to be from surroundings
fn start_pipe(grid: &[Vec<Tile>], sx: usize, sy: usize) -> Tile {
    let (m, n) = (grid.len(), grid.first().unwrap().len());
    if sx > 0 && sx < m - 1 && grid[sx - 1][sy].accepts(S) && grid[sx + 1][sy].accepts(N) {
        NS
    } else if sy > 0 && sy < n - 1 && grid[sx][sy - 1].accepts(E) && grid[sx][sy + 1].accepts(W) {
        EW
    } else if sx > 0 && sy < n - 1 && grid[sx - 1][sy].accepts(S) && grid[sx][sy + 1].accepts(W) {
        NE
    } else if sx > 0 && sy > 0 && grid[sx - 1][sy].accepts(S) && grid[sx][sy - 1].accepts(E) {
        NW
    } else if sx < m - 1 && sy > 0 && grid[sx][sy - 1].accepts(E) && grid[sx + 1][sy].accepts(N) {
        SW
    } else if sx < m - 1 && sy < n - 1 && grid[sx][sy + 1].accepts(W) && grid[sx + 1][sy].accepts(N)
    {
        SE
    } else {
        panic!("Impossible start pipe")
    }
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut grid: Vec<Vec<Tile>> = vec![];
    while let Some(line) = scan.read_line() {
        grid.push(line.chars().map(Tile::from_char).collect());
    }
    let (m, n) = (grid.len(), grid.first().unwrap().len());

    // find start cell
    let (mut sx, mut sy) = (0, 0);
    for (i, line) in grid.iter().enumerate() {
        for (j, &t) in line.iter().enumerate() {
            if t == Start {
                (sx, sy) = (i, j);
                break;
            }
        }
    }

    let s_pipe = start_pipe(&grid, sx, sy);
    grid[sx][sy] = s_pipe;

    // maintain grid of enclosed, loop, and outside cells
    let mut grid_status = vec![vec![Outside; n]; m];
    grid_status[sx][sy] = Loop;

    let mut res_1 = 1;
    let (mut x, mut y, mut from_dir) = match s_pipe {
        NS => (sx - 1, sy, S),
        EW => (sx, sy + 1, W),
        NE => (sx - 1, sy, S),
        NW => (sx - 1, sy, S),
        SE => (sx + 1, sy, N),
        SW => (sx + 1, sy, N),
        _ => unreachable!(),
    };

    // move along path
    while x != sx || y != sy {
        unsafe {
            *grid_status.get_unchecked_mut(x).get_unchecked_mut(y) = Loop;
        }
        (x, y, from_dir) = match grid[x][y] {
            NS => match from_dir {
                N => (x + 1, y, N),
                S => (x - 1, y, S),
                _ => unreachable!(),
            },
            EW => match from_dir {
                E => (x, y - 1, E),
                W => (x, y + 1, W),
                _ => unreachable!(),
            },
            NE => match from_dir {
                N => (x, y + 1, W),
                E => (x - 1, y, S),
                _ => unreachable!(),
            },
            NW => match from_dir {
                N => (x, y - 1, E),
                W => (x - 1, y, S),
                _ => unreachable!(),
            },
            SW => match from_dir {
                S => (x, y - 1, E),
                W => (x + 1, y, N),
                _ => unreachable!(),
            },
            SE => match from_dir {
                S => (x, y + 1, W),
                E => (x + 1, y, N),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        res_1 += 1;
    }

    // res_1 is length of path, divide by 1 and round up for steps to furthest
    res_1 = (res_1 + 1) / 2;

    // find parity of vertical bars and L and J, cells are inside if parity is odd
    let res_2 = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .fold((0, 0), |(mut acc, mut parity), (j, &tile)| {
                    if unsafe { *grid_status.get_unchecked(i).get_unchecked(j) == Loop } {
                        parity ^= match tile {
                            NS | NE | NW => 1,
                            _ => 0,
                        };
                    } else if parity % 2 == 1 {
                        acc += 1
                    }
                    (acc, parity)
                })
                .0
        })
        .sum::<i32>();
    write!(out, "{}\n{}\n", res_1, res_2);

    //grid_status.iter().for_each(|r| {
    //    println!(
    //        "{}",
    //        r.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("")
    //    )
    //});

    let mut grid_3 = vec![];
    for i in 0..m {
        let mut t = vec![];
        let mut m = vec![];
        let mut b = vec![];
        for j in 0..n {
            if unsafe { *grid_status.get_unchecked(i).get_unchecked(j) == Loop } {
                match *unsafe { grid.get_unchecked(i).get_unchecked(j) } {
                    NS => {
                        t.push(Loop); t.push(Enclosed); t.push(Loop); 
                        m.push(Loop); m.push(Enclosed); m.push(Loop); 
                        b.push(Loop); b.push(Enclosed); b.push(Loop); 
                    }
                    EW => {
                        t.push(Enclosed); t.push(Enclosed); t.push(Enclosed);
                        m.push(Loop); m.push(Loop); m.push(Loop);
                        b.push(Enclosed); b.push(Enclosed); b.push(Enclosed);
                    }
                    NE => {
                        t.push(Loop); t.push(Enclosed); t.push(Enclosed);
                        m.push(Loop); m.push(Enclosed); m.push(Enclosed);
                        b.push(Loop); b.push(Loop); b.push(Loop);
                    }
                    NW => {
                        t.push(Enclosed); t.push(Loop); t.push(Enclosed);
                        m.push(Loop); m.push(Loop); m.push(Enclosed);
                        b.push(Enclosed); b.push(Enclosed); b.push(Enclosed);
                    }
                    SW => {
                        t.push(Enclosed); t.push(Enclosed); t.push(Enclosed);
                        m.push(Loop); m.push(Loop); m.push(Enclosed);
                        b.push(Enclosed); b.push(Loop); b.push(Enclosed);
                    }
                    SE => {
                        t.push(Enclosed); t.push(Enclosed); t.push(Enclosed);
                        m.push(Enclosed); m.push(Loop); m.push(Loop);
                        b.push(Enclosed); b.push(Loop); b.push(Enclosed);
                    },
                    _ => unreachable!()
                }
            }
            else {
                t.push(Enclosed); t.push(Enclosed); t.push(Enclosed);
                m.push(Enclosed); m.push(Enclosed); m.push(Enclosed);
                b.push(Enclosed); b.push(Enclosed); b.push(Enclosed);
            }
        }
        grid_3.push(t);
        grid_3.push(m);
        grid_3.push(b);
    }
    
    // assume (0, 0) is not inside loop
    let (m3, n3) = (m * 3, n * 3);
    let mut st = vec![(0,0)];
    while let Some((i, j)) = st.pop() {
        unsafe {
            match *grid_3.get_unchecked(i).get_unchecked(j) {
                Enclosed => {
                    *grid_3.get_unchecked_mut(i).get_unchecked_mut(j) = Outside;
                    if i > 0 { st.push((i - 1, j)); }
                    if i < m3 - 1 { st.push((i + 1, j)); }
                    if j > 0 { st.push((i, j - 1)); }
                    if j < n3 - 1 { st.push((i, j + 1)); }
                },
                Loop => continue,
                Outside => continue,
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            unsafe {
                if *grid_3.get_unchecked(3 * i).get_unchecked(3 * j) == Enclosed &&
                *grid_3.get_unchecked(3 * i).get_unchecked(3 * j+1) == Enclosed &&
                *grid_3.get_unchecked(3 * i).get_unchecked(3 * j+2) == Enclosed &&
                *grid_3.get_unchecked(3 * i+1).get_unchecked(3 * j) == Enclosed &&
                *grid_3.get_unchecked(3 * i+1).get_unchecked(3 * j+1) == Enclosed &&
                *grid_3.get_unchecked(3 * i+1).get_unchecked(3 * j+2) == Enclosed &&
                *grid_3.get_unchecked(3 * i+2).get_unchecked(3 * j) == Enclosed &&
                *grid_3.get_unchecked(3 * i+2).get_unchecked(3 * j+1) == Enclosed &&
                *grid_3.get_unchecked(3 * i+2).get_unchecked(3 * j+1) == Enclosed {
                    *grid_status.get_unchecked_mut(i).get_unchecked_mut(j) = Enclosed;
                }
            }
        }
    }

    //grid_status.iter().for_each(|r| {
    //    println!(
    //        "{}",
    //        r.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("")
    //    )
    //});
    //
    //println!();
    //
    //grid_3.iter().for_each(|r| {
    //    println!(
    //        "{}",
    //        r.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("")
    //    )
    //});
}
