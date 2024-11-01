// There are n people standing in a circle waiting to be executed. The counting out begins at
// some point in the circle and proceeds around the circle in a fixed direction. In each step,
// a certain number of people are skipped and the next person is executed. The elimination
// proceeds around the circle (which is becoming smaller and smaller as the executed people
// are removed), until only the last person remains, who is given freedom.

// Given the total number of persons n and a number k which indicates that k-1 persons are skipped and
// the kth person is killed in a circle. The task is to choose the person in the initial circle that survives.

pub fn josephus(n: usize, k: usize) -> usize {
    if n == 1 {
        // since n is 1 so there is only one person remaining in the circle which has id 0
        return 0;
    }

    (josephus(n - 1, k) + k) % n
}

#[cfg(test)]
mod tests {
    use super::*;
    // id of first person in circle is 0
    // id of last person in circle is n-1

    #[test]
    fn test1() {
        assert_eq!(josephus(5, 2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(josephus(7, 3), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(josephus(7, 4), 1);
    }
}
