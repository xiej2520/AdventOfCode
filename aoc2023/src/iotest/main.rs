#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufRead, BufWriter, stdin, stdout, Write};


pub fn main() {
    let mut scan = UnsafeScanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());
    
    let n: i32 = scan.token();
    write!(out, "{}\n", n);
    for _ in 0..n {
        let s: String = scan.token();
        writeln!(out, "{}", s);
    }
    for _ in 0..n {
        let a: i32 = scan.token();
        writeln!(out, "{}", a);
    }
    for _ in 0..n {
        let l: f64 = scan.token();
        writeln!(out, "{:.8}", l);
    }
}
// with -C opt-level=3
//real    0m1.234s
//user    0m0.762s
//sys     0m0.471s
//writeln! same speed


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
