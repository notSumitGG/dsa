// For given two integers, swap their values using bitwise operators

pub fn swap(mut a: usize, mut b: usize) -> (usize, usize) {
    a = a ^ b;
    b = a ^ b;
    a = a ^ b;

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(swap(4, 5), (5, 4));
    }
}
