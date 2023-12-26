#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::{UnsafeScanner, DSU};
use counter::Counter;
use rand::seq::SliceRandom;
use rustc_hash::FxHashMap;
use std::io::{stdin, stdout, BufWriter, Write};

fn karger(n: usize, edges: &Vec<(usize, usize)>) -> (DSU, usize) {
    fn trial(n: usize, edges: &Vec<(usize, usize)>) -> (DSU, usize) {
        let mut comps = n;
        let mut dsu = DSU::new(n);
        let mut shuffled = edges.clone();
        shuffled.shuffle(&mut rand::thread_rng());
        while comps > 2 {
            let (a, b) = shuffled.pop().unwrap();
            let a = dsu.find(a);
            let b = dsu.find(b);
            if a != b {
                dsu.union(a, b);
                comps -= 1;
            }
        }
        //println!("remaining edges {}/{}", shuffled.len(), edges.len());

        let mut res = 0;
        for &(i, j) in edges {
            let a = dsu.find(i);
            let b = dsu.find(j);
            if a != b {
                // need to cut
                res += 1;
            }
        }
        (dsu, res)
    }

    let mut res = trial(n, edges);
    //let mut times = 1;
    while res.1 != 3 {
        res = trial(n, edges);
        //times += 1;
    }
    //println!("took {times} attempts. Nodes {n}, edges: {}", edges.len());
    res
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut comp_map = FxHashMap::default();
    let mut adj = vec![];
    let mut edges = vec![];
    while let Some(line) = scan.read_line() {
        let mut sp = line.split(": ");
        let a = sp.next().unwrap().to_owned();
        let a = match comp_map.get(&a) {
            Some(&id) => id,
            None => {
                comp_map.insert(a, comp_map.len());
                adj.push(vec![]);
                comp_map.len() - 1
            }
        };
        for b in sp.next().unwrap().split(' ') {
            let b = b.to_owned();
            let b = match comp_map.get(&b) {
                Some(&id) => id,
                None => {
                    comp_map.insert(b, comp_map.len());
                    adj.push(vec![]);
                    comp_map.len() - 1
                }
            };
            adj[a].push(b);
            adj[b].push(a);
            edges.push((a, b));
        }
    }
    let n = adj.len();
    let (dsu, _) = karger(n, &edges);
    // returns a dsu with find() ran on every node
    let counts: Counter<_> = dsu.parent.iter().collect();
    let res_1: usize = counts.values().product();
    let res_2 = "";

    write!(out, "{}\n{}\n", res_1, res_2);
}
