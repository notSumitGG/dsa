pub fn transpose<T: Copy>(m: &[impl AsRef<[T]>]) -> Vec<Vec<T>> {
    let mut result = vec![Vec::with_capacity(m.len()); m[0].as_ref().len()];
    dbg!(m.len());
    dbg!(m[0].as_ref().len());

    for i in 0..m[0].as_ref().len() {
        for j in 0..m.len() {
            result[i].push(m[j].as_ref().iter().nth(i).unwrap().to_owned());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            transpose(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }
}
