const QUEEN: u8 = 81; // 'Q'
const EMPTY: u8 = 46; // '.'

pub fn nqueen(size: usize) -> Vec<Vec<String>> {
    let mut board = vec![vec![EMPTY; size]; size];
    let mut solutions = Vec::<Vec<String>>::new();
    solve_nqueen(0, size, &mut board, &mut solutions);
    solutions
}

fn solve_nqueen(
    row: usize,
    size: usize,
    board: &mut Vec<Vec<u8>>,
    solutions: &mut Vec<Vec<String>>,
) {
    if row == size {
        solutions.push(
            board
                .iter()
                .map(|row| row.iter().map(|c| *c as char).collect())
                .collect(),
        );
        return;
    }

    for col in 0..size {
        if is_safe(row, col, size, &board) {
            board[row][col] = QUEEN;
            solve_nqueen(row + 1, size, board, solutions);
            board[row][col] = EMPTY;
        }
    }
}

fn is_safe(row: usize, col: usize, size: usize, board: &Vec<Vec<u8>>) -> bool {
    for i in 0..row {
        if board[i][col] == QUEEN
            || (col >= (row - i) && board[i][col - (row - i)] == QUEEN)
            || (col + row - i < size && board[i][col + (row - i)] == QUEEN)
        {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(nqueen(0), vec![Vec::<String>::new()]);
    }

    #[test]
    fn test1() {
        assert_eq!(nqueen(1), vec![vec!["Q"]]);
    }

    #[test]
    fn test2() {
        assert_eq!(nqueen(2), Vec::<Vec<String>>::new());
    }

    #[test]
    fn test3() {
        assert_eq!(nqueen(3), Vec::<Vec<String>>::new());
    }

    #[test]
    fn test4() {
        assert_eq!(
            nqueen(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            nqueen(5),
            vec![
                vec!["Q....", "..Q..", "....Q", ".Q...", "...Q."],
                vec!["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
                vec![".Q...", "...Q.", "Q....", "..Q..", "....Q"],
                vec![".Q...", "....Q", "..Q..", "Q....", "...Q."],
                vec!["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
                vec!["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
                vec!["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
                vec!["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
                vec!["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
                vec!["....Q", "..Q..", "Q....", "...Q.", ".Q..."],
            ]
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            nqueen(6),
            vec![
                vec![".Q....", "...Q..", ".....Q", "Q.....", "..Q...", "....Q."],
                vec!["..Q...", ".....Q", ".Q....", "....Q.", "Q.....", "...Q.."],
                vec!["...Q..", "Q.....", "....Q.", ".Q....", ".....Q", "..Q..."],
                vec!["....Q.", "..Q...", "Q.....", ".....Q", "...Q..", ".Q...."],
            ]
        );
    }
}
