pub fn fast_power(mut x: u128, mut n: u128) -> u128 {
    let mut result = 1;
    while n > 0 {
        if n & 1 == 1 {
            result *= x;
        }
        x *= x;
        n >>= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(fast_power(2, 8), 256);
    }
    #[test]
    fn test2() {
        assert_eq!(fast_power(3, 6), 729);
    }
}
