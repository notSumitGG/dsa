// Binary search is a technique to find the a given element in a an array within O(log(n)) amortized time,
// which must be sorted in ascending or descending order

pub fn binary_search(v: &[usize], find: usize, mut lb: usize, mut ub: usize) -> Option<usize> {
    let mut mid: usize;
    while lb <= ub {
        mid = lb + (ub - lb) / 2;
        if v[mid] < find {
            lb = mid + 1;
        } else if v[mid] > find {
            ub = mid - 1;
        } else {
            return Some(mid);
        }
    }
    return None;
}

pub fn binary_search_recursive(v: &[usize], find: usize, lb: usize, ub: usize) -> Option<usize> {
    if lb > ub {
        return None;
    }

    let mid = lb + (ub - lb) / 2;
    if v[mid] < find {
        binary_search_recursive(v, find, mid + 1, ub)
    } else if find < v[mid] {
        binary_search_recursive(v, find, lb, mid - 1)
    } else {
        Some(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 0, 9),
            Some(9)
        );
        assert_eq!(
            binary_search_recursive(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 0, 9),
            Some(9)
        );
    }
}
