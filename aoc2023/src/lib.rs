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
