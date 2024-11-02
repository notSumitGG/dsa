// a majority element is an element which occurs more than floor(n/2) number of times
// where n = length of the array

pub fn majority_element(v: &[i64]) -> Option<i64> {
    let mut majority = v[0];
    let mut count = 1usize;
    let n = v.len();
    for i in 1..n {
        if v[i] == majority {
            count += 1
        } else {
            count -= 1;
            if count == 0 {
                majority = v[i];
                count = 1;
            }
        }
    }

    // extra step for returning correct answer
    // if this is not done the above code will return candidate majority element
    if count != 1 {
        Some(majority)
    } else {
        let mut c: usize = 0;
        for i in 0..n {
            if v[i] == majority {
                c += 1;
            }
        }
        if c > (n / 2) {
            Some(majority)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(majority_element(&vec![1, 2, 1, 3, 1, 1]), Some(1));
    }
}
