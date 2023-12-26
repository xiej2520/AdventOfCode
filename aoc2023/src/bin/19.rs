#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};

use aoc2023::UnsafeScanner;
use rustc_hash::FxHashMap;
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::from_utf8_unchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    X(i64),
    M(i64),
    A(i64),
    S(i64),
    N,
}
use Rating::*;

#[derive(Debug)]
struct Factor {
    rating: Rating,
    greater_than: bool,
    next: Vec<u8>,
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let mut workflow_map = FxHashMap::default();
    while let Some(line) = scan.read_line() {
        if line.is_empty() {
            break;
        }
        let bytes = line.as_bytes();
        let mut i = 0;
        while bytes[i] != b'{' {
            i += 1;
        }
        let name = &bytes[0..i];
        let rules_bytes = &bytes[i + 1..bytes.len() - 1];
        let mut rules = vec![];
        for rule in rules_bytes.split(|&c| c == b',') {
            let mut i = 0;
            while i < rule.len() && rule[i] != b':' {
                i += 1;
            }
            if i == rule.len() {
                rules.push(Factor {
                    rating: N,
                    greater_than: false,
                    next: rule.to_vec(),
                });
            } else {
                let next_name = &rule[i + 1..];
                let val: i64 = unsafe { from_utf8_unchecked(&rule[2..i]).parse().unwrap() };
                rules.push(Factor {
                    rating: match rule[0] {
                        b'x' => X(val),
                        b'm' => M(val),
                        b'a' => A(val),
                        b's' => S(val),
                        _ => unreachable!(),
                    },
                    greater_than: rule[1] == b'>',
                    next: next_name.to_vec(),
                });
            }
        }
        workflow_map.insert(name.to_vec(), rules);
    }

    let mut res_1 = 0;
    while let Some(line) = scan.read_line() {
        let bytes = line.as_bytes();
        #[derive(Default, Debug)]
        struct Part {
            x: i32,
            m: i32,
            a: i32,
            s: i32,
        }
        let mut p = Part::default();
        for rating in bytes[1..bytes.len() - 1].split(|&c| c == b',') {
            let value: i32 = unsafe { from_utf8_unchecked(&rating[2..]).parse().unwrap() };
            match rating.first().unwrap() {
                b'x' => p.x = value,
                b'm' => p.m = value,
                b'a' => p.a = value,
                b's' => p.s = value,
                _ => unreachable!(),
            }
        }
        let mut cur_workflow = "in".as_bytes().to_vec();
        let eval = |rating, gt, val| {
            let val = val as i32;
            if gt {
                rating > val
            } else {
                rating < val
            }
        };
        while cur_workflow.len() > 1 {
            for rule in &workflow_map[&cur_workflow] {
                match rule.rating {
                    X(val) => {
                        if eval(p.x, rule.greater_than, val) {
                            cur_workflow = rule.next.clone();
                            break;
                        }
                    }
                    M(val) => {
                        if eval(p.m, rule.greater_than, val) {
                            cur_workflow = rule.next.clone();
                            break;
                        }
                    }
                    A(val) => {
                        if eval(p.a, rule.greater_than, val) {
                            cur_workflow = rule.next.clone();
                            break;
                        }
                    }
                    S(val) => {
                        if eval(p.s, rule.greater_than, val) {
                            cur_workflow = rule.next.clone();
                            break;
                        }
                    }
                    N => {
                        cur_workflow = rule.next.clone();
                        break;
                    }
                }
            }
        }
        if *cur_workflow.first().unwrap() == b'A' {
            res_1 += p.x + p.m + p.a + p.s;
        }
    }

    fn dfs(
        workflows: &FxHashMap<Vec<u8>, Vec<Factor>>,
        cur_workflow: &[u8],
        mut x: (i64, i64),
        mut m: (i64, i64),
        mut a: (i64, i64),
        mut s: (i64, i64),
    ) -> i64 {
        if *cur_workflow.first().unwrap() == b'R'
            || x.1 <= x.0
            || m.1 <= m.0
            || a.1 <= a.0
            || s.1 <= s.0
        {
            return 0;
        }
        if *cur_workflow.first().unwrap() == b'A' {
            return (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0);
        }
        let mut res = 0;
        for rule in &workflows[cur_workflow] {
            match rule.rating {
                X(val) => {
                    // max and min clamping isn't necessary for the AoC inputs,
                    // those workflows don't have redundant rules
                    if rule.greater_than {
                        res += dfs(workflows, &rule.next, (max(x.0, val + 1), x.1), m, a, s);
                        x.1 = val + 1;
                    } else {
                        res += dfs(workflows, &rule.next, (x.0, min(x.1, val)), m, a, s);
                        x.0 = val;
                    }
                }
                M(val) => {
                    if rule.greater_than {
                        res += dfs(workflows, &rule.next, x, (max(m.0, val + 1), m.1), a, s);
                        m.1 = val + 1;
                    } else {
                        res += dfs(workflows, &rule.next, x, (m.0, min(m.1, val)), a, s);
                        m.0 = val;
                    }
                }
                A(val) => {
                    if rule.greater_than {
                        res += dfs(workflows, &rule.next, x, m, (max(a.0, val + 1), a.1), s);
                        a.1 = val + 1;
                    } else {
                        res += dfs(workflows, &rule.next, x, m, (a.0, min(a.1, val)), s);
                        a.0 = val;
                    }
                }
                S(val) => {
                    if rule.greater_than {
                        res += dfs(workflows, &rule.next, x, m, a, (max(s.0, val + 1), s.1));
                        s.1 = val + 1;
                    } else {
                        res += dfs(workflows, &rule.next, x, m, a, (s.0, min(s.1, val)));
                        s.0 = val;
                    }
                }
                N => res += dfs(workflows, &rule.next, x, m, a, s),
            }
        }
        res
    }

    let res_2 = dfs(
        &workflow_map,
        "in".as_bytes(),
        (1, 4001),
        (1, 4001),
        (1, 4001),
        (1, 4001),
    );
    write!(out, "{}\n{}\n", res_1, res_2);
}
