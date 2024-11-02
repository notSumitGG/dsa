// For a given rotated array and a query element find the index of element

// for ascending order
pub fn rotated_array_binary_search(v: &[usize], find: usize) -> Option<usize> {
    let (mut lb, mut ub) = (0, v.len() - 1);
    let mut mid: usize;
    while lb <= ub {
        mid = lb + (ub - lb) / 2;

        if v[mid] == find {
            return Some(mid);
        }

        if v[mid] <= v[ub] {
            // pivot on left
            if v[mid] < find && find <= v[ub] {
                // sorted part on right
                lb = mid + 1;
            } else {
                ub = mid - 1;
            }
        } else if v[lb] <= v[mid] {
            // pivot on right
            if v[lb] <= find && find < v[mid] {
                // sorted part on left
                ub = mid - 1;
            } else {
                lb = mid + 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            rotated_array_binary_search(&[21, 32, 1, 4, 7, 11, 15, 20], 7),
            Some(4)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            rotated_array_binary_search(&[7, 11, 15, 20, 21, 32, 1, 4], 32),
            Some(5)
        );
    }
}
