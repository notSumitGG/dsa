pub fn prime_factorization(mut n: i64) -> Vec<(i64, usize)> {
    let mut factors = Vec::<(i64, usize)>::new();
    let mut fact = 2;

    while n != 1 {
        let mut factcount = 0;
        while n % fact == 0 {
            factcount += 1;
            n /= fact;
        }

        if factcount > 0 {
            factors.push((fact, factcount));
        }
        fact += 1;
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(prime_factorization(12), vec![(2, 2), (3, 1)]);
    }
}
