pub fn grid_paths(row: usize, col: usize) -> usize {
    if row == 1 || col == 1 {
        return 1;
    }

    grid_paths(row - 1, col) + grid_paths(row, col - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(grid_paths(3, 3), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(grid_paths(4, 5), 35);
    }
}
