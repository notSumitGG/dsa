pub fn matrix_addition<T: std::ops::Add<Output = T> + Copy>(
    x: &[impl AsRef<[T]>],
    y: &[impl AsRef<[T]>],
) -> Result<Vec<Vec<T>>, &'static str> {
    if x.len() != y.len() || x[0].as_ref().len() != y[0].as_ref().len() {
        return Err("Error: Matrices order doesn't match");
    }

    Ok(std::iter::zip(x, y)
        .map(|(rx, ry)| {
            std::iter::zip(rx.as_ref(), ry.as_ref())
                .map(|(cx, cy)| *cx + *cy)
                .collect()
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            matrix_addition::<i32>(&[[1, 3, 5], [2, 4, 6]], &[[-2, -4, -6], [-1, -3, -5]]),
            Ok(vec![vec![-1, -1, -1], vec![1, 1, 1]])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            matrix_addition::<i64>(
                &vec![vec![1, 2, 3], vec![4, 5, 6]],
                &[vec![0, 10, 20], vec![100, 200, 300]]
            ),
            Ok(vec![vec![1, 12, 23], vec![104, 205, 306]])
        );
    }
}
