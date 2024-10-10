//Given an array of integers. All numbers occur even no of times except one number which occurs odd number of times.

pub fn odd_times_occuring_element(v: &[i64]) -> i64 {
    let mut result = v[0];
    for i in 1..v.len() {
        result ^= v[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(
            odd_times_occuring_element(&[3, 4, 6, 8, 4, 6, 2, 2, 8, 3, 3]),
            3
        );
    }
}
