// https://leetcode.com/problems/n-queens
//
// The **n-queens** puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no two queens attack each other.
//
// Given an integer `n`, return _all distinct solutions to the **n-queens puzzle**_. You may return the answer in **any order**.
//
// Each solution contains a distinct board configuration of the n-queens' placement, where `'Q'` and `'.'` both indicate a queen and an empty space, respectively.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
// ```
// **Input:** n = 4
// **Output:** [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
// **Explanation:** There exist two distinct solutions to the 4-queens puzzle as shown above
// ```
//
// **Example 2:**
//
// ```
// **Input:** n = 1
// **Output:** [["Q"]]
// ```
//
// **Constraints:**
//
// *   `1 <= n <= 9`

// https://www.youtube.com/watch?v=Ph95IHmRp5M
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut col = std::collections::HashSet::new();
    let mut pos_diag = std::collections::HashSet::new();
    let mut neg_diag = std::collections::HashSet::new();
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    fn backtrace(
        n: i32,
        col: &mut std::collections::HashSet<i32>,
        pos_diag: &mut std::collections::HashSet<i32>,
        neg_diag: &mut std::collections::HashSet<i32>,
        board: &mut Vec<Vec<char>>,
        r: i32,
        res: &mut Vec<Vec<String>>,
    ) {
        if r == n {
            res.push(
                board
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect(),
            );
            return;
        }
        for c in 0..n {
            if col.contains(&c) || pos_diag.contains(&(r + c)) || neg_diag.contains(&(r - c)) {
                continue;
            }
            col.insert(c);
            pos_diag.insert(r + c);
            neg_diag.insert(r - c);
            board[r as usize][c as usize] = 'Q';

            backtrace(n, col, pos_diag, neg_diag, board, r + 1, res);

            col.remove(&c);
            pos_diag.remove(&(r + c));
            neg_diag.remove(&(r - c));
            board[r as usize][c as usize] = '.';
        }
    }
    let mut res = Vec::new();
    backtrace(
        n,
        &mut col,
        &mut pos_diag,
        &mut neg_diag,
        &mut board,
        0,
        &mut res,
    );
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        solve_n_queens(4),
        vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string()
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string()
            ],
        ]
    );
}
