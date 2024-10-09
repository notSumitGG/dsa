// Binary search is a technique to find the a given element in a an array within O(log(n)) amortized time,
// which must be sorted in ascending or descending order

pub fn binary_search(a: &[usize], find: usize, mut left: usize, mut right: usize) -> Option<usize> {
    let mut mid: usize;
    while left <= right {
        mid = left + (right - left) / 2;
        if a[mid] < find {
            left = mid + 1;
        } else if a[mid] > find {
            right = mid - 1;
        } else {
            return Some(mid);
        }
    }
    return None;
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

        let index = binary_search(&a, find, 0, n);
        if let Some(i) = index {
            println!("{} found at index {}", find, i);
        } else {
            println!("{} not found", find);
        }
    }
}

/*
10 8
1 2 3 4 5 6 7 8 9 10
*/
