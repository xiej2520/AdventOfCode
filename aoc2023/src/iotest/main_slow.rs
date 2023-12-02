#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufRead, BufWriter, stdin, stdout, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();

    let n: i32 = scan.next();
    print!("{}\n", n);
    for _ in 0..n {
        let s: String = scan.next();
        print!("{}", s);
    }
    for _ in 0..n {
        let a: i32 = scan.next();
        print!("{}", a);
    }
    for _ in 0..n {
        let l: f64 = scan.next();
        print!("{:.8}", l);
    }
    
}
// with -C opt-level=3
//real    0m2.527s
//user    0m1.858s
//sys     0m0.670s


/// https://github.com/EbTech/rust-algorithms/blob/master/src/scanner.rs
/// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
