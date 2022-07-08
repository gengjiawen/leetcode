// https://leetcode.com/problems/special-positions-in-a-binary-matrix
//
// Given an `m x n` binary matrix `mat`, return _the number of special positions in_ `mat`_._
//
// A position `(i, j)` is called **special** if `mat[i][j] == 1` and all other elements in row `i` and column `j` are `0` (rows and columns are **0-indexed**).
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2021/12/23/special1.jpg)
// ```
// **Input:** mat = [[1,0,0],[0,0,1],[1,0,0]]
// **Output:** 1
// **Explanation:** (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2021/12/24/special-grid.jpg)
// ```
// **Input:** mat = [[1,0,0],[0,1,0],[0,0,1]]
// **Output:** 3
// **Explanation:** (0, 0), (1, 1) and (2, 2) are special positions.
// ```
//
// **Constraints:**
//
// *   `m == mat.length`
// *   `n == mat[i].length`
// *   `1 <= m, n <= 100`
// *   `mat[i][j]` is either `0` or `1`.

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    fn is_special(mat: &Vec<Vec<i32>>, p1: usize, p2: usize) -> bool {
        for i in 0..mat.len() {
            if i == p1 {
                continue;
            }
            if mat[i][p2] == 1 {
                return false;
            }
        }
        for j in 0..mat[0].len() {
            if j == p2 {
                continue;
            }
            if mat[p1][j] == 1 {
                return false;
            }
        }
        return true;
    }
    let mut count = 0;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 1 && is_special(&mat, i, j) {
                count += 1;
            }
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(
        num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
        1
    );
}
