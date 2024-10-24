mod backtracking;
mod bit_manipulation;
mod combinatorics;
mod number_theory;
mod searching;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1() {
//         assert_eq!(
//         );
//     }
// }

// fn main() {
//     use std::io::Write;
//     let mut token = Scanner::new(std::io::stdin().lock());
//     let mut out = std::io::BufWriter::new(std::io::stdout().lock());
// }

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<u8>,
    iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
            iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.iter.next() {
                return unsafe { token.parse().unwrap_unchecked() };
            }
            self.buffer.clear();
            self.reader.read_until(b'\n', &mut self.buffer).unwrap();
            self.iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
