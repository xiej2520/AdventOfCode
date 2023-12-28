#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{UnsafeScanner, Grid};
use bigdecimal::BigDecimal;
use bnum::{BInt, types::{I256, I128}};
use num_traits::{Zero, FromPrimitive};
use std::{
    io::{stdin, stdout, BufWriter, Write},
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct Coord2f {
    x: f64,
    y: f64,
}

impl Add for Coord2f {
    type Output = Coord2f;

    fn add(self, rhs: Self) -> Self::Output {
        Coord2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Mul<f64> for Coord2f {
    type Output = Coord2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Coord2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Obj {
    p: Coord3,
    v: Coord3,
}

fn intersect(a1: Coord2f, a2: Coord2f, b1: Coord2f, b2: Coord2f) -> Option<Coord2f> {
    let den = (a1.x - a2.x) * (b1.y - b2.y) - (a1.y - a2.y) * (b1.x - b2.x);
    if den == 0.0 {
        None
    } else {
        let det_1_2 = a1.x * a2.y - a1.y * a2.x;
        let det_3_4 = b1.x * b2.y - b1.y * b2.x;
        Some(Coord2f {
            x: (det_1_2 * (b1.x - b2.x) - (a1.x - a2.x) * det_3_4) / den,
            y: (det_1_2 * (b1.y - b2.y) - (a1.y - a2.y) * det_3_4) / den,
        })
    }
}

fn proj(a: Coord3) -> Coord2f {
    Coord2f {
        x: a.x as f64,
        y: a.y as f64,
    }
}

// augmented matrix A
// must have one solution, otherwise will divide by zero or give wrong answer
fn gauss(mut A: Grid<BigDecimal>) -> Vec<BigDecimal> {
    debug_assert_eq!(A.m() + 1, A.n());
    let m = A.m();
    
    // transform into row echelon form
    for i in 0..m-1 {
        // row swap
        if A[(i,i)].abs() < BigDecimal::from_f64(0.000000001).unwrap() {
            for l in i+1..m {
                if !A[(l,i)].is_zero() {
                    //println!("Swapping rows {} and {}",i,l);
                    for j in i..m+1 {
                        unsafe {
                            debug_assert_ne!(l, i);
                            std::ptr::swap(&mut A[(i, j)], &mut A[(l, j)]);
                        }
                    }
                    break;
                }
            }
        }
        let f = A[(i, i)].clone();
        for j in i..m+1 {
            A[(i, j)] = &A[(i, j)] / &f;
        }
        for j in i..m-1 {
            if A[(i,i)].is_zero() {
                continue;
            }
            //println!("{} {}",i,j);
            let f = &A[(j+1,i)] / &A[(i,i)];
            for k in i..m+1 {
                let t = &f * &A[(i,k)];
                A[(j+1,k)] -= t;
            }
        }
    //for (i,row) in A.iter().enumerate() {
    //    print!("{}: ",i);
    //    row.iter().for_each(|d| print!("{:.8}, ", d));
    //    println!();
    //    //println!("{:?}", row);
    //}
    //println!();
    }
    //println!("row echelon");
    
    for i in (1..m).rev() {
        if A[(i,i)].is_zero() {
            continue;
        }
        if A[(i,i)].abs() < BigDecimal::from_f64(1e-15).unwrap() {
            continue;
        }
        for j in (1..i+1).rev() {
            //println!("{} {}",i,j);
            let f = &A[(j-1,i)] / &A[(i,i)];
            for k in (0..m+1).rev() {
                let t = &f * &A[(i,k)];
                A[(j-1,k)] -= t;
            }
        }
    //for (i,row) in A.iter().enumerate() {
    //    print!("{}: ",i);
    //    row.iter().for_each(|d| print!("{:.8}, ", d));
    //    println!();
    //    //println!("{:?}", row);
    //}
    //println!();
    }

    let mut res = vec![];
    
    //println!();
    //println!();
    //for (i,row) in A.iter().enumerate() {
    //    print!("{}: ",i);
    //    row.iter().for_each(|d| print!("{:.8}, ", d));
    //    println!();
    //    //println!("{:?}", row);
    //}
    //println!();
    //println!();
    for i in 0..m {
        res.push(&A[(i, m)] / &A[(i, i)]);
    }
    
    res
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut hailstones = vec![];
    while let Some(line) = scan.read_line() {
        let mut sp = line.split(" @ ");
        let mut sp_pt = sp.next().unwrap().split(", ");
        let p = Coord3 {
            x: sp_pt.next().unwrap().parse().unwrap(),
            y: sp_pt.next().unwrap().parse().unwrap(),
            z: sp_pt.next().unwrap().parse().unwrap(),
        };
        let mut sp_pt = sp.next().unwrap().split(", ");
        let v = Coord3 {
            x: sp_pt.next().unwrap().parse().unwrap(),
            y: sp_pt.next().unwrap().parse().unwrap(),
            z: sp_pt.next().unwrap().parse().unwrap(),
        };
        hailstones.push(Obj { p, v });
    }
    let n = hailstones.len();

    const L: f64 = 200000000000000.0;
    const H: f64 = 400000000000000.0;
    let mut res_1 = 0;
    for i in 0..n {
        let ap = proj(hailstones[i].p);
        let av = proj(hailstones[i].v);
        for j in i + 1..n {
            let bp = proj(hailstones[j].p);
            let bv = proj(hailstones[j].v);
            // floating point precision error, multiply by 100
            // could do int math in intersect before the divide, but needs big ints
            if let Some(Coord2f { x, y }) = intersect(ap, ap + av * 100.0, bp, bp + bv * 100.0) {
                // back in time
                if (x - ap.x) / av.x < 0.0 || (y - ap.y) / av.y < 0.0 {
                    continue;
                }
                if (x - bp.x) / bv.x < 0.0 || (y - bp.y) / bv.y < 0.0 {
                    continue;
                }
                if x < L || y < L || x > H || y > H {
                    continue;
                }
                res_1 += 1;
            }
        }
    }
    
    let Coord3 {x: x1, y: y1, z: z1} = hailstones[0].p;
    let Coord3 {x: x2, y: y2, z: z2} = hailstones[1].p;
    let Coord3 {x: x3, y: y3, z: z3} = hailstones[2].p;
    let Coord3 {x: u1, y: v1, z: w1} = hailstones[0].v;
    let Coord3 {x: u2, y: v2, z: w2} = hailstones[1].v;
    let Coord3 {x: u3, y: v3, z: w3} = hailstones[2].v;

    let x1 = BigDecimal::from(x1);
    let x2 = BigDecimal::from(x2);
    let x3 = BigDecimal::from(x3);
    let y1 = BigDecimal::from(y1);
    let y2 = BigDecimal::from(y2);
    let y3 = BigDecimal::from(y3);
    let z1 = BigDecimal::from(z1);
    let z2 = BigDecimal::from(z2);
    let z3 = BigDecimal::from(z3);
    let u1 = BigDecimal::from(u1);
    let u2 = BigDecimal::from(u2);
    let u3 = BigDecimal::from(u3);
    let v1 = BigDecimal::from(v1);
    let v2 = BigDecimal::from(v2);
    let v3 = BigDecimal::from(v3);
    let w1 = BigDecimal::from(w1);
    let w2 = BigDecimal::from(w2);
    let w3 = BigDecimal::from(w3);
    
    let Z = BigDecimal::zero();
    let M = vec![
        vec![Z.clone(), &w1-&w2, -&v1+&v2, Z.clone(), -&z1+&z2, &y1-&y2,
        &y1*&w1 - &z1*&v1 - &y2*&w2 + &z2*&v2,
        ],
        vec![-&w1+&w2, Z.clone(), &u1-&u2, &z1-&z2, Z.clone(), -&x1+&x2,
        &z1*&u1 - &x1*&w1 - &z2*&u2 + &x2*&w2,
        ],
        vec![&v1-&v2, -&u1+&u2, Z.clone(), -&y1+&y2, &x1-&x2, Z.clone(),
        &x1*&v1 - &y1*&u1 - &x2*&v2 + &y2*&u2,
        ],
        vec![Z.clone(), &w1-&w3, -&v1+&v3, Z.clone(), -&z1+&z3, &y1-&y3,
        &y1*&w1 - &z1*&v1 - &y3*&w3 + &z3*&v3,
        ],
        vec![-&w1+&w3, Z.clone(), &u1-&u3, &z1-&z3, Z.clone(), -&x1+&x3,
        &z1*&u1 - &x1*&w1 - &z3*&u3 + &x3*&w3,
        ],
        vec![&v1-&v3, -&u1+&u3, Z.clone(), -&y1+&y3, &x1-&x3, Z.clone(),
        &x1*&v1 - &y1*&u1 - &x3*&v3 + &y3*&u3,
        ],
    ];
    let A = Grid::from_2d(M);
    //for row in A.iter() {
    //    println!("  {:?}",row);
    //}
    
    //std::env::set_var("RUST_BACKTRACE", "1");
    let res = gauss(A);
    //println!("{:?}",res);
    let res_2 = &res[0] + &res[1] + &res[2];


    write!(out, "{}\n{}\n", res_1, res_2);
}
