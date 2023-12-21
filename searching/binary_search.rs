enum Res {
    Found(usize),
    NotFound
}

fn bin_search(mut left: usize, mut right: usize, a: &Vec<usize>, find: usize) -> Res {
    let mut mid: usize;
    while left <= right {
        mid = left + (right - left) / 2;
        if a[mid] < find {
            left = mid + 1;
        } else if a[mid] > find {
            right = mid - 1;
        } else {
            return Res::Found(mid);
        }
    }
    return Res::NotFound;
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let find: usize = token.next();
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(token.next());
    }

    let index = bin_search(0, n, &a, find);
    if let Res::Found(i) = index {
        println!("{} found at index {}", find, i);
    } else {
        println!("{} not found", find);
    }
}

/* 
10 8
1 2 3 4 5 6 7 8 9 10
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
