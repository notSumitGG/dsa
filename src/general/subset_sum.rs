// Given an array set[] of non-negative integers and a value sum, the task is to
// check if there is a subset of the given array whose sum is equal to the given sum.

pub fn subset_sum(set: &[isize], target: isize) -> bool {
    subset_sum_recursive(set, target, set.len())
}

fn subset_sum_recursive(set: &[isize], target: isize, items_left: usize) -> bool {
    if target == 0 {
        return true;
    }

    if items_left == 0 {
        return false;
    }

    subset_sum_recursive(set, target, items_left - 1)
        || subset_sum_recursive(set, target - set[items_left - 1], items_left - 1)
}

pub fn subset_sum_iterative(set: &[isize], target: isize) -> bool {
    for mask in 0..(1 << set.len()) {
        let mut sum_of_subset = 0isize;
        for i in 0..set.len() {
            if mask & (1 << i) != 0 {
                sum_of_subset += set[i];
            }
        }

        if sum_of_subset == target {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(subset_sum(&[3, 34, 4, 12, 5, 2], 9), true);
        assert_eq!(subset_sum_iterative(&[3, 34, 4, 12, 5, 2], 9), true);
    }

    #[test]
    fn test2() {
        assert_eq!(subset_sum(&[3, 34, 4, 12, 5, 2], 30), false);
        assert_eq!(subset_sum_iterative(&[3, 34, 4, 12, 5, 2], 30), false);
    }

    #[test]
    fn test3() {
        assert_eq!(subset_sum(&[1, 2, 3, 4, 5, 6], 10,), true);
        assert_eq!(subset_sum_iterative(&[1, 2, 3, 4, 5, 6], 10), true);
    }

    #[test]
    fn test4() {
        assert_eq!(
            subset_sum(&[5, 10, 12, 13, 15, 18, -1, 10, 50, -2, 3, 4], 30),
            true
        );
        assert_eq!(
            subset_sum_iterative(&[5, 10, 12, 13, 15, 18, -1, 10, 50, -2, 3, 4], 30),
            true
        );
    }
}
