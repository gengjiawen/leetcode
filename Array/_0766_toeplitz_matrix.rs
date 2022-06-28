// https://leetcode.com/problems/toeplitz-matrix
//
// Given an `m x n` `matrix`, return _`true` if the matrix is Toeplitz. Otherwise, return `false`._
//
// A matrix is **Toeplitz** if every diagonal from top-left to bottom-right has the same elements.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2020/11/04/ex1.jpg)
// ```
// **Input:** matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
// **Output:** true
// **Explanation:**
// In the above grid, the diagonals are:
// "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
// In each diagonal all elements are the same, so the answer is True.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2020/11/04/ex2.jpg)
// ```
// **Input:** matrix = [[1,2],[2,2]]
// **Output:** false
// **Explanation:**
// The diagonal "[1, 2]" has different elements.
// ```
//
// **Constraints:**
//
// *   `m == matrix.length`
// *   `n == matrix[i].length`
// *   `1 <= m, n <= 20`
// *   `0 <= matrix[i][j] <= 99`
//
// **Follow up:**
//
// *   What if the `matrix` is stored on disk, and the memory is limited such that you can only load at most one row of the matrix into the memory at once?
// *   What if the `matrix` is so large that you can only load up a partial row into the memory at once?

pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if i > 0 && j > 0 && matrix[i][j] != matrix[i - 1][j - 1] {
                return false;
            }
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(
        is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
        true
    );
}
