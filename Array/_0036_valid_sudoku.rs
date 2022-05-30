// https://leetcode.com/problems/valid-sudoku
//
// Determine if a `9 x 9` Sudoku board is valid. Only the filled cells need to be validated **according to the following rules**:
//
// 1.  Each row must contain the digits `1-9` without repetition.
// 2.  Each column must contain the digits `1-9` without repetition.
// 3.  Each of the nine `3 x 3` sub-boxes of the grid must contain the digits `1-9` without repetition.
//
// **Note:**
//
// *   A Sudoku board (partially filled) could be valid but is not necessarily solvable.
// *   Only the filled cells need to be validated according to the mentioned rules.
//
// **Example 1:**
//
// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
// ```
// **Input:** board =
// [["5","3",".",".","7",".",".",".","."]
// ,["6",".",".","1","9","5",".",".","."]
// ,[".","9","8",".",".",".",".","6","."]
// ,["8",".",".",".","6",".",".",".","3"]
// ,["4",".",".","8",".","3",".",".","1"]
// ,["7",".",".",".","2",".",".",".","6"]
// ,[".","6",".",".",".",".","2","8","."]
// ,[".",".",".","4","1","9",".",".","5"]
// ,[".",".",".",".","8",".",".","7","9"]]
// **Output:** true
// ```
//
// **Example 2:**
//
// ```
// **Input:** board =
// [["8","3",".",".","7",".",".",".","."]
// ,["6",".",".","1","9","5",".",".","."]
// ,[".","9","8",".",".",".",".","6","."]
// ,["8",".",".",".","6",".",".",".","3"]
// ,["4",".",".","8",".","3",".",".","1"]
// ,["7",".",".",".","2",".",".",".","6"]
// ,[".","6",".",".",".",".","2","8","."]
// ,[".",".",".","4","1","9",".",".","5"]
// ,[".",".",".",".","8",".",".","7","9"]]
// **Output:** false
// **Explanation:** Same as Example 1, except with the **5** in the top left corner being modified to **8**. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
// ```
//
// **Constraints:**
//
// *   `board.length == 9`
// *   `board[i].length == 9`
// *   `board[i][j]` is a digit `1-9` or `'.'`.

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![std::collections::HashSet::new(); 9];
    let mut columns = vec![std::collections::HashSet::new(); 9];
    let mut squars = vec![std::collections::HashSet::new(); 9];
    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }
            if !rows[i].insert(c) {
                return false;
            }
            if !columns[j].insert(c) {
                return false;
            }
            let k = (i / 3) * 3 + j / 3;
            if !squars[k].insert(c) {
                return false;
            }
        }
    }

    return true;
}

#[test]
fn test() {
    let board = vec![
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
    assert_eq!(is_valid_sudoku(board), true);
    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(is_valid_sudoku(board), false);
}
