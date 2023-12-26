#![allow(unused_must_use)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::cmp::{max, min};
use std::collections::VecDeque;

use aoc2023::UnsafeScanner;
use rustc_hash::FxHashMap;
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Debug, Clone)]
enum Module {
    // dest, is_on
    FlipFlop(Vec<u8>, bool),
    Conjunction(Vec<u8>, ConjunctionInputs),
    Broadcast(Vec<u8>),
}
use Module::*;

impl Module {
    fn get_dest_mut(&mut self) -> &mut Vec<u8> {
        match self {
            FlipFlop(dest, _) => dest,
            Conjunction(dest, _) => dest,
            Broadcast(dest) => dest,
        }
    }
    fn get_dest(&self) -> &Vec<u8> {
        match self {
            FlipFlop(dest, _) => dest,
            Conjunction(dest, _) => dest,
            Broadcast(dest) => dest,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct ConjunctionInputs {
    inputs: FxHashMap<u8, bool>,
    num_high: u8,
}

fn push_button(modules: &mut [Module], broadcaster: u8) -> (i32, i32) {
    let mut q = VecDeque::new();
    q.push_back((255, broadcaster, false));

    let (mut low_sent, mut high_sent) = (0, 0);
    while !q.is_empty() {
        let (send, recv, high) = q.pop_front().unwrap();
        if high {
            high_sent += 1
        } else {
            low_sent += 1
        }

        match &mut modules[recv as usize] {
            FlipFlop(dest_vec, is_on) => {
                if !high {
                    *is_on = !*is_on;
                    for &mut dest in dest_vec {
                        q.push_back((recv, dest, *is_on));
                    }
                }
            }
            Conjunction(dest_vec, ConjunctionInputs { inputs, num_high }) => {
                let prev_input = inputs.get_mut(&send).unwrap();
                if *prev_input != high {
                    if high {
                        *num_high += 1;
                    } else {
                        *num_high -= 1;
                    }
                    *prev_input = high;
                }
                let send_high = *num_high != inputs.len() as u8;
                for &mut dest in dest_vec {
                    q.push_back((recv, dest, send_high));
                }
            }
            Broadcast(dest_vec) => {
                for &mut dest in dest_vec {
                    q.push_back((recv, dest, high));
                }
            }
        }
    }
    (low_sent, high_sent)
}

fn push_button_2(
    modules: &mut [Module],
    broadcaster: u8,
    listen_high: &[u8],
    listen_recv: u8,
) -> Option<u8> {
    let mut q = VecDeque::new();
    q.push_back((255, broadcaster, false));

    let mut res = None;
    while !q.is_empty() {
        let (send, recv, high) = q.pop_front().unwrap();
        if high && recv == listen_recv && listen_high.contains(&send) {
            res = Some(send);
        }

        match &mut modules[recv as usize] {
            FlipFlop(dest_vec, is_on) => {
                if !high {
                    *is_on = !*is_on;
                    for &mut dest in dest_vec {
                        q.push_back((recv, dest, *is_on));
                    }
                }
            }
            Conjunction(dest_vec, ConjunctionInputs { inputs, num_high }) => {
                let prev_input = inputs.get_mut(&send).unwrap();
                if *prev_input != high {
                    if high {
                        *num_high += 1;
                    } else {
                        *num_high -= 1;
                    }
                    *prev_input = high;
                }
                let send_high = *num_high != inputs.len() as u8;
                for &mut dest in dest_vec {
                    q.push_back((recv, dest, send_high));
                }
            }
            Broadcast(dest_vec) => {
                for &mut dest in dest_vec {
                    q.push_back((recv, dest, high));
                }
            }
        }
    }
    res
}

pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    // name -> id
    let mut module_map = FxHashMap::default();
    let mut modules = vec![];

    let mut module_count = 0;
    while let Some(line) = scan.read_line() {
        let mut sp = line.split(" -> ");

        let module_name_f = sp.next().unwrap().trim_end().as_bytes();

        let first_char = *module_name_f.first().unwrap();
        let module_name = match first_char {
            b'%' | b'&' => &module_name_f[1..],
            _ => module_name_f,
        }
        .to_vec();

        let cur_id = match module_map.get(&module_name) {
            Some(&id) => id,
            None => {
                module_map.insert(module_name, module_count);
                module_count += 1;
                modules.push(Broadcast(vec![]));
                module_count - 1 // cur_id
            }
        };

        let mut module = match first_char {
            b'%' => FlipFlop(vec![], false),
            b'&' => Conjunction(vec![], Default::default()),
            _ => Broadcast(vec![]),
        };

        let destinations = sp.next().unwrap();
        for dest in destinations.split(", ") {
            let dest = dest.as_bytes().to_vec();
            let dest_id = match module_map.get(&dest) {
                Some(&id) => id,
                None => {
                    module_map.insert(dest, module_count);
                    module_count += 1;
                    modules.push(Broadcast(vec![]));
                    module_count - 1
                }
            };
            module.get_dest_mut().push(dest_id);
        }
        modules[cur_id as usize] = module;
    }

    // I <3 mutable aliasing
    for i in 0..modules.len() {
        let num_dest = modules[i].get_dest().len();
        for j in 0..num_dest {
            let dest_id = modules[i].get_dest()[j] as usize;
            if let Conjunction(
                _,
                ConjunctionInputs {
                    inputs,
                    num_high: _,
                },
            ) = &mut modules[dest_id]
            {
                inputs.insert(i as u8, false);
            }
        }
    }

    let broadcaster = module_map["broadcaster".as_bytes()];
    let (mut low_pulses, mut high_pulses) = (0, 0);
    for _ in 0..1000 {
        let (l, h) = push_button(&mut modules, broadcaster);
        low_pulses += l;
        high_pulses += h;
    }

    let res_1 = low_pulses * high_pulses;

    let mut reset = || {
        for module in &mut modules {
            match module {
                FlipFlop(_, is_on) => {
                    *is_on = false;
                }
                Conjunction(_, ConjunctionInputs { inputs, num_high }) => {
                    for v in inputs.values_mut() {
                        *v = false;
                    }
                    *num_high = 0;
                }
                Broadcast(_) => {}
            }
        }
    };
    reset();

    let mut res_2 = 1;
    // rx's input is a conjunction module
    // it will send a low pulse when it receives a high pulse from all its inputs
    let rx_id = module_map["rx".as_bytes()] as usize;
    let mut rx_input_id = 0;
    'outer: for (i, module) in modules.iter().enumerate() {
        for dest in module.get_dest() {
            if *dest == rx_id as u8 {
                rx_input_id = i;
                break 'outer;
            }
        }
    }
    let mut rx_input_high: FxHashMap<u8, i64> = if let Conjunction(
        _,
        ConjunctionInputs {
            inputs,
            num_high: _,
        },
    ) = &modules[rx_input_id]
    {
        inputs.iter().map(|(&input, _)| (input, 0)).collect()
    } else {
        unreachable!();
    };

    let listen_inputs: Vec<u8> = rx_input_high.iter().map(|(&k, _)| k).collect();

    let mut i = 1;
    while rx_input_high.iter().any(|(_, &v)| v == 0) {
        if let Some(input) =
            push_button_2(&mut modules, broadcaster, &listen_inputs, rx_input_id as u8)
        {
            rx_input_high.insert(input, i);
        }
        i += 1;
    }
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    for (_, v) in rx_input_high {
        res_2 = res_2 / gcd(res_2, v) * v;
    }

    write!(out, "{}\n{}\n", res_1, res_2);
}
