// https://leetcode.com/problems/sudoku-solver
//
// Write a program to solve a Sudoku puzzle by filling the empty cells.
//
// A sudoku solution must satisfy **all of the following rules**:
//
// 1.  Each of the digits `1-9` must occur exactly once in each row.
// 2.  Each of the digits `1-9` must occur exactly once in each column.
// 3.  Each of the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
//
// The `'.'` character indicates empty cells.
//
// **Example 1:**
//
// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
// ```
// **Input:** board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
// **Output:** [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
// **Explanation:** The input board is shown above and the only valid solution is shown below:
//
// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png)
// ```
//
// **Constraints:**
//
// *   `board.length == 9`
// *   `board[i].length == 9`
// *   `board[i][j]` is a digit or `'.'`.
// *   It is **guaranteed** that the input board has only one solution.

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn is_valid_point(
        board: &Vec<Vec<char>>,
        row: usize,
        column: usize,
        current_point: char,
    ) -> bool {
        for i in 0..9 {
            if board[i][column] != '.' && board[i][column] == current_point {
                return false;
            }
            if board[row][i] != '.' && board[row][i] == current_point {
                return false;
            }
            let (r, c) = (row / 3 * 3 + i / 3, column / 3 * 3 + i % 3);
            if board[r][c] != '.' && board[r][c] == current_point {
                return false;
            }
        }
        return true;
    }
    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for k in 0..9 {
                        let char = (b'1' + k) as char;
                        if is_valid_point(board, i, j, char) {
                            board[i][j] = char;
                            if backtrack(board) {
                                return true;
                            } else {
                                board[i][j] = '.';
                            }
                        }
                    }
                    return false;
                }
            }
        }

        return true;
    }
    backtrack(board);
}

#[test]
pub fn t1() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    solve_sudoku(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ]
    );
}

#[test]
pub fn t2() {
    let a = 1;
    let b = (b'1' + a) as char;
    assert_eq!(b, '2');
}
