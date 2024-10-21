pub fn grid_paths_recursive(row: usize, col: usize) -> usize {
    if row == 1 || col == 1 {
        return 1;
    }

    grid_paths_recursive(row - 1, col) + grid_paths_recursive(row, col - 1)
}

fn grid_paths_dp(dp: &mut Vec<Vec<Option<usize>>>, row: usize, col: usize) -> usize {
    if let Some(val) = dp[row][col] {
        return val;
    }

    dp[row - 1][col] = Some(grid_paths_dp(dp, row - 1, col));
    dp[row][col - 1] = Some(grid_paths_dp(dp, row, col - 1));
    return dp[row - 1][col].unwrap() + dp[row][col - 1].unwrap();
}

pub fn grid_paths(row: usize, col: usize) -> usize {
    let mut dp = vec![vec![None; col + 1]; row + 1];
    for i in 1..col + 1 {
        dp[1][i] = Some(1);
    }
    for i in 1..row + 1 {
        dp[i][1] = Some(1);
    }
    grid_paths_dp(&mut dp, row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(grid_paths_recursive(3, 3), 6);
        assert_eq!(grid_paths(3, 3), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(grid_paths_recursive(4, 5), 35);
        assert_eq!(grid_paths(4, 5), 35);
    }

    #[test]
    fn test3() {
        assert_eq!(grid_paths_recursive(5, 4), 35);
        assert_eq!(grid_paths(5, 4), 35);
    }
}
