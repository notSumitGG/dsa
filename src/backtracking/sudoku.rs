pub fn solve_sudoku(mut board: [[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    if solve(&mut board) {
        Some(board)
    } else {
        None
    }
}

fn solve(board: &mut [[u8; 9]; 9]) -> bool {
    let empty_cell = find_empty_cell(board);

    if let Some((row, column)) = empty_cell {
        for value in 1..=9 {
            if is_value_valid(&board, (row, column), value) {
                board[row][column] = value;
                if solve(board) {
                    return true;
                }
                // Backtracking if the board cannot be solved using the current configuration
                board[row][column] = 0;
            }
        }
    } else {
        // If the board is complete
        return true;
    }

    // Returning false if the board cannot be solved using the current configuration
    false
}

fn find_empty_cell(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    // Find an empty cell in the board (returns None if all cells are filled)
    for row in 0..9 {
        for column in 0..9 {
            if board[row][column] == 0 {
                return Some((row, column));
            }
        }
    }

    None
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
