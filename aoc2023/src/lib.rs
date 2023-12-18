use std::io::BufRead;

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

impl<R: BufRead> UnsafeScanner<R> {
    pub fn read_line(&mut self) -> Option<String> {
        if self
            .reader
            .read_until(b'\n', &mut self.buf_str)
            .expect("Failed read")
            == 0
        {
            None
        } else {
            if *self.buf_str.last().unwrap() == b'\n' {
                self.buf_str.pop();
            }
            unsafe {
                Some(String::from_utf8_unchecked(std::mem::take(
                    &mut self.buf_str,
                )))
            }
        }
    }
}

pub struct BIT {
    bit: Vec<i32>,
}
#[allow(dead_code)]
impl BIT {
    pub fn new(n: usize) -> Self {
        BIT { bit: vec![0; n] }
    }
    pub fn sum(&self, mut r: i32) -> i32 {
        assert!(r >= 0 && r < self.bit.len() as i32);
        let mut res = 0;
        while r >= 0 {
            res += unsafe { self.bit.get_unchecked(r as usize) };
            r = (r & (r + 1)) - 1;
        }
        res
    }
    pub fn add(&mut self, mut index: usize, delta: i32) {
        assert!(index < self.bit.len());
        while index < self.bit.len() {
            unsafe {
                *self.bit.get_unchecked_mut(index) += delta;
            }
            index |= index + 1;
        }
    }
}

pub struct SegTree {
    s: Vec<i32>,
    n: usize,
}
#[allow(dead_code)]
impl SegTree {
    pub fn new(n: usize) -> Self {
        SegTree {
            s: vec![0; 2 * n],
            n,
        }
    }
    pub fn replace(&mut self, mut pos: usize, val: i32) {
        assert!(pos < self.n);
        unsafe {
            pos += self.n;
            *self.s.get_unchecked_mut(pos) = val;
            pos >>= 1;
            while pos != 0 {
                *self.s.get_unchecked_mut(pos) =
                    *self.s.get_unchecked(pos * 2) + *self.s.get_unchecked(pos * 2 + 1);

                pos >>= 1;
            }
        }
    }
    pub fn add(&mut self, mut pos: usize, val: i32) {
        assert!(pos < self.n);
        unsafe {
            pos += self.n;
            *self.s.get_unchecked_mut(pos) += val;
            pos >>= 1;
            while pos != 0 {
                *self.s.get_unchecked_mut(pos) =
                    *self.s.get_unchecked(pos * 2) + *self.s.get_unchecked(pos * 2 + 1);

                pos >>= 1;
            }
        }
    }
    pub fn query(&self, mut l: usize, mut r: usize) -> i32 {
        assert!(l < self.n);
        assert!(r < self.n);
        let (mut ra, mut rb) = (0, 0);
        l += self.n;
        r += self.n + 1;
        while l < r {
            if l % 2 == 1 {
                ra += unsafe { *self.s.get_unchecked(l) };
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                rb += unsafe { *self.s.get_unchecked(r) };
            }
            l >>= 1;
            r >>= 1;
        }
        ra + rb
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

// returns solution x to system of congruences x \equiv a_i \mod m_i
pub fn crt(congruences: &[(i64, i64)]) -> i64 {
    let prod_m: i64 = congruences.iter().map(|c| c.1).product();

    congruences.iter().fold(0, |acc, &(a, m)| {
        let m_i = prod_m / m;
        let n_i = mod_inv(m_i, m).unwrap();
        (acc + a * m_i % prod_m * n_i) % prod_m
    })
}

mod grid;
pub use crate::grid::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crt() {
        assert_eq!(23, super::crt(&[(2, 3), (3, 5), (2, 7)]));
    }
}
