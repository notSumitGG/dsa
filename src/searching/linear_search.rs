// Linear Searching is a brute force method of searching an element in a array with
// amortized time of O(n)

pub fn linear_search(v: &[usize], find: usize, left: usize, right: usize) -> Option<usize> {
    for i in left..right {
        if v[i] == find {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut token = crate::Scanner::new(std::io::stdin().lock());
        let n: usize = token.next();
        let find: usize = token.next();
        let mut a: Vec<usize> = Vec::new();
        for _ in 0..n {
            a.push(token.next());
        }

        let index = linear_search(&a, find, 0, n);
        if let Some(i) = index {
            println!("{} found at index {}", find, i);
        } else {
            println!("{} not found", find);
        }
    }
}
