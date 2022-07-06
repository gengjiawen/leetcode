// https://leetcode.com/problems/transpose-matrix
//
// Given a 2D integer array `matrix`, return _the **transpose** of_ `matrix`.
//
// The **transpose** of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
//
// ![](https://assets.leetcode.com/uploads/2021/02/10/hint_transpose.png)
//
// **Example 1:**
//
// ```
// **Input:** matrix = [[1,2,3],[4,5,6],[7,8,9]]
// **Output:** [[1,4,7],[2,5,8],[3,6,9]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** matrix = [[1,2,3],[4,5,6]]
// **Output:** [[1,4],[2,5],[3,6]]
// ```
//
// **Constraints:**
//
// *   `m == matrix.length`
// *   `n == matrix[i].length`
// *   `1 <= m, n <= 1000`
// *   `1 <= m * n <= 10<sup>5</sup>`
// *   `-10<sup>9</sup> <= matrix[i][j] <= 10<sup>9</sup>`

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            res[j][i] = matrix[i][j];
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
}
