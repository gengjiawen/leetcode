// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix
//
// Given a `m x n` matrix `grid` which is sorted in non-increasing order both row-wise and column-wise, return _the number of **negative** numbers in_ `grid`.
//
// **Example 1:**
//
// ```
// **Input:** grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
// **Output:** 8
// **Explanation:** There are 8 negatives number in the matrix.
// ```
//
// **Example 2:**
//
// ```
// **Input:** grid = [[3,2],[1,0]]
// **Output:** 0
// ```
//
// **Constraints:**
//
// *   `m == grid.length`
// *   `n == grid[i].length`
// *   `1 <= m, n <= 100`
// *   `-100 <= grid[i][j] <= 100`
//
// **Follow up:** Could you find an `O(n + m)` solution?

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in grid {
        for (index, cell) in row.iter().enumerate() {
            if *cell < 0 {
                count = count + row.len() as i32 - index as i32;
                break;
            }
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(
        count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
}
