// Linear Searching is a brute force method of searching an element in a array with
// amortized time of O(n)

pub fn linear_search(v: &[usize], find: usize, lb: usize, ub: usize) -> Option<usize> {
    for i in lb..ub {
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
    fn test1() {
        assert_eq!(linear_search(&[3, 9, 0, 23, 83, 9], 9, 0, 6), Some(1));
    }
}
