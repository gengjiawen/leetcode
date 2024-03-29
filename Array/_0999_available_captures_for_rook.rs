// https://leetcode.com/problems/available-captures-for-rook
//
// On an `8 x 8` chessboard, there is **exactly one** white rook `'R'` and some number of white bishops `'B'`, black pawns `'p'`, and empty squares `'.'`.
//
// When the rook moves, it chooses one of four cardinal directions (north, east, south, or west), then moves in that direction until it chooses to stop, reaches the edge of the board, captures a black pawn, or is blocked by a white bishop. A rook is considered **attacking** a pawn if the rook can capture the pawn on the rook's turn. The **number of available captures** for the white rook is the number of pawns that the rook is **attacking**.
//
// Return _the **number of available captures** for the white rook_.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2019/02/20/1253_example_1_improved.PNG)
// ```
// **Input:** board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","R",".",".",".","p"],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
// **Output:** 3
// **Explanation:** In this example, the rook is attacking all the pawns.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2019/02/19/1253_example_2_improved.PNG)
// ```
// **Input:** board = [[".",".",".",".",".",".",".","."],[".","p","p","p","p","p",".","."],[".","p","p","B","p","p",".","."],[".","p","B","R","B","p",".","."],[".","p","p","B","p","p",".","."],[".","p","p","p","p","p",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
// **Output:** 0
// **Explanation:** The bishops are blocking the rook from attacking any of the pawns.
// ```
//
// **Example 3:**
//
// ![](https://assets.leetcode.com/uploads/2019/02/20/1253_example_3_improved.PNG)
// ```
// **Input:** board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","p",".",".",".","."],["p","p",".","R",".","p","B","."],[".",".",".",".",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."]]
// **Output:** 3
// **Explanation:** The rook is attacking the pawns at positions b5, d6, and f5.
// ```
//
// **Constraints:**
//
// *   `board.length == 8`
// *   `board[i].length == 8`
// *   `board[i][j]` is either `'R'`, `'.'`, `'B'`, or `'p'`
// *   There is exactly one cell with `board[i][j] == 'R'`

pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut rook_x = 0;
    let mut rook_y = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'R' {
                rook_x = i;
                rook_y = j;
                break;
            }
        }
    }
    let mut count = 0;
    for i in (0..rook_x).rev() {
        if board[i][rook_y] == 'B' {
            break;
        } else if board[i][rook_y] == 'p' {
            count += 1;
            break;
        }
    }

    for i in (rook_x + 1)..board.len() {
        if board[i][rook_y] == 'B' {
            break;
        } else if board[i][rook_y] == 'p' {
            count += 1;
            break;
        }
    }

    for i in (0..rook_y).rev() {
        if board[rook_x][i] == 'B' {
            break;
        } else if board[rook_x][i] == 'p' {
            count += 1;
            break;
        }
    }
    for i in (rook_y + 1)..board[rook_x].len() {
        if board[rook_x][i] == 'B' {
            break;
        } else if board[rook_x][i] == 'p' {
            count += 1;
            break;
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(
        num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
}
