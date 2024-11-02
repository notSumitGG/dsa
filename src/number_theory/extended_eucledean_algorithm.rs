pub fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut div, mut rem) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);

    while rem != 0 {
        let quotient = div / rem;

        update_value(&mut div, &mut rem, quotient);
        update_value(&mut s1, &mut s2, quotient);
        update_value(&mut t1, &mut t2, quotient);
    }

    (div, s1, t1)
}

fn update_value(v1: &mut i64, v2: &mut i64, quotient: i64) {
    let temp = *v2;
    *v2 = *v1 - quotient * temp;
    *v1 = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(extended_euclidean_algorithm(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean_algorithm(13, 101), (1, -31, 4));
    }

    #[test]
    fn test2() {
        assert_eq!(extended_euclidean_algorithm(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean_algorithm(19, 123), (1, 13, -2));
    }

    #[test]
    fn test3() {
        assert_eq!(extended_euclidean_algorithm(50, 70), (10, 3, -2));
        assert_eq!(extended_euclidean_algorithm(70, 50), (10, -2, 3));
    }

    #[test]
    fn test4() {
        assert_eq!(extended_euclidean_algorithm(33, 44), (11, -1, 1));
    }

    #[test]
    fn test5() {
        assert_eq!(extended_euclidean_algorithm(55, 79), (1, 23, -16));
    }

    #[test]
    fn test6() {
        assert_eq!(extended_euclidean_algorithm(25, 36), (1, 13, -9));
    }

    #[test]
    fn test7() {
        assert_eq!(extended_euclidean_algorithm(69, 54), (3, -7, 9));
    }
}
