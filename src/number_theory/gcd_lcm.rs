pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

pub fn compute_for_array(v: &[i64]) -> (i64, i64) {
    let (mut gcd_val, mut lcm_val) = (v[0], v[0]);

    for i in 1..v.len() {
        gcd_val = gcd(gcd_val, v[i]);
        lcm_val = lcm(lcm_val, v[i]);
    }

    (gcd_val, lcm_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(compute_for_array(&[1, 2, 3, 4, 6, 12]), (1, 12));
    }
}
