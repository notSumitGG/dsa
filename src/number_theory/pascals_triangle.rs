pub fn generate_pascal(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::<Vec<usize>>::with_capacity(n);
    result.push(vec![1]);

    for i in 1..=n {
        let mut line = Vec::<usize>::with_capacity(i + 1);
        line.push(1);

        let mut c = 1usize;
        for r in 1..=i {
            c = c * (i - r + 1) / r;
            line.push(c);
        }

        result.push(line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            generate_pascal(4),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
