// a majority element is an element which occurs more than floor(n/2) number of times
// where n = length of the array

fn majority_element(n: usize, v: &[i64]) -> Option<i64> {
    let mut majority = v[0];
    let mut count = 1usize;
    for i in 1..n {
        if v[i] == majority {
            count += 1
        } else {
            count -= 1;
            if count == 0 {
                majority = v[i];
                count = 1;            
            }
        }
    }

    // extra step for returning correct answer
    // if this is not done the above code will return candidate majority element
    if count != 1 {
        Some(majority)
    } else {
        let mut c: usize = 0;
        for i in 0..n {
            if v[i] == majority {
                c += 1;
            }
        }
        if c > (n/2) {
            Some(majority)
        } else {
            None
        }
    }
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let mut v: Vec<i64> = Vec::with_capacity(n);
    for _ in 0usize..n {
        v.push(token.next());
    }

    let r = majority_element(n, &v);
    if let Some(val) = r {
        println!("Majority Element: {}", val);
    } else {
        println!("Majority Element does not exists");
    }
}

/*
6
1 2 1 3 1 1
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

