// Given a partially filled 9×9 2D array ‘grid[9][9]’, the goal is to assign
// digits (from 1 to 9) to the empty cells so that every row, column, and
// subgrid of size 3×3 contains exactly one instance of the digits from 1 to 9.

pub fn solve_sudoku(mut board: [[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    if solve(&mut board, 0, 0) {
        Some(board)
    } else {
        None
    }
}

fn solve(board: &mut [[u8; 9]; 9], mut row: usize, mut col: usize) -> bool {
    if row == 8 && col == 9 {
        return true;
    }

    if col == 9 {
        row += 1;
        col = 0;
    }

    if board[row][col] > 0 {
        return solve(board, row, col + 1);
    }

    for i in 1..=9 {
        if is_value_valid(&board, (row, col), i) {
            board[row][col] = i;
            if solve(board, row, col + 1) {
                return true;
            }
            board[row][col] = 0;
        }
    }

    false
}

fn is_value_valid(board: &[[u8; 9]; 9], coordinates: (usize, usize), value: u8) -> bool {
    let (row, column) = coordinates;

    // Checks if the value to be added in the board is an acceptable value for the cell
    for i in 0..9 {
        // Checking through the row
        if board[row][i] == value {
            return false;
        }
        // Checking through the column
        if board[i][column] == value {
            return false;
        }
    }

    // Checking through the 3x3 block of the cell
    let start_row = (row / 3) * 3;
    let start_column = (column / 3) * 3;

    for current_row in start_row..start_row + 3 {
        for current_column in start_column..start_column + 3 {
            if board[current_row][current_column] == value {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            solve_sudoku([
                [3, 0, 6, 5, 0, 8, 4, 0, 0],
                [5, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 8, 7, 0, 0, 0, 0, 3, 1],
                [0, 0, 3, 0, 1, 0, 0, 8, 0],
                [9, 0, 0, 8, 6, 3, 0, 0, 5],
                [0, 5, 0, 0, 9, 0, 6, 0, 0],
                [1, 3, 0, 0, 0, 0, 2, 5, 0],
                [0, 0, 0, 0, 0, 0, 0, 7, 4],
                [0, 0, 5, 2, 0, 6, 3, 0, 0],
            ]),
            Some([
                [3, 1, 6, 5, 7, 8, 4, 9, 2],
                [5, 2, 9, 1, 3, 4, 7, 6, 8],
                [4, 8, 7, 6, 2, 9, 5, 3, 1],
                [2, 6, 3, 4, 1, 5, 9, 8, 7],
                [9, 7, 4, 8, 6, 3, 1, 2, 5],
                [8, 5, 1, 7, 9, 2, 6, 4, 3],
                [1, 3, 8, 9, 4, 7, 2, 5, 6],
                [6, 9, 2, 3, 5, 1, 8, 7, 4],
                [7, 4, 5, 2, 8, 6, 3, 1, 9],
            ])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve_sudoku([
                [6, 0, 3, 5, 0, 8, 4, 0, 0],
                [5, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 8, 7, 0, 0, 0, 0, 3, 1],
                [0, 0, 3, 0, 1, 0, 0, 8, 0],
                [9, 0, 0, 8, 6, 3, 0, 0, 5],
                [0, 5, 0, 0, 9, 0, 6, 0, 0],
                [1, 3, 0, 0, 0, 0, 2, 5, 0],
                [0, 0, 0, 0, 0, 0, 0, 7, 4],
                [0, 0, 5, 2, 0, 6, 3, 0, 0],
            ]),
            // None::<[[u8; 9]; 9]>
            Option::<[[u8; 9]; 9]>::None
        );
    }
}
