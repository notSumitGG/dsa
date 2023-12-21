fn kadenes_algorithm(v: &[i64], n: usize) -> (usize, usize, i64) {
    let mut maxsum: i64 = v[0];
    let mut cursum: i64 = v[0];
    let mut l: usize = 0;
    let mut r: usize = 0;
    for i in 1usize..n {
        cursum += v[i];
        if cursum > maxsum {
            maxsum = cursum;
            r = i;
        }
        if cursum < 0 {
            cursum = 0;
            l = i+1;
            r = i+1;
        }
    }
    (l, r, maxsum)
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let mut v: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(token.next());
    }
    let (l, r, sum) = kadenes_algorithm(&v, n);
    println!("[{}..={}] = {}", l, r, sum);
}

/*
8
1 3 -4 8 2 -5 0 10
*/

struct Tokenizer {
    buf: Vec<String>,
    i: usize
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer { buf: Vec::<String>::new(), i: 0 };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T : std::str::FromStr>(&mut self) -> T
    where T::Err : std::fmt::Debug {
        while self.i == self.buf.len() {
            self.read_line();
        }
        let t = self.buf[self.i].parse().unwrap();
        self.i += 1;
        return t;
    }

    #[allow(dead_code)]
    pub fn next_line(&self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        return s;
    }
}

