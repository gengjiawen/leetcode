// https://leetcode.com/problems/matrix-cells-in-distance-order
//
// You are given four integers `row`, `cols`, `rCenter`, and `cCenter`. There is a `rows x cols` matrix and you are on the cell with the coordinates `(rCenter, cCenter)`.
//
// Return _the coordinates of all cells in the matrix, sorted by their **distance** from_ `(rCenter, cCenter)` _from the smallest distance to the largest distance_. You may return the answer in **any order** that satisfies this condition.
//
// The **distance** between two cells `(r<sub>1</sub>, c<sub>1</sub>)` and `(r<sub>2</sub>, c<sub>2</sub>)` is `|r<sub>1</sub> - r<sub>2</sub>| + |c<sub>1</sub> - c<sub>2</sub>|`.
//
// **Example 1:**
//
// ```
// **Input:** rows = 1, cols = 2, rCenter = 0, cCenter = 0
// **Output:** [[0,0],[0,1]]
// **Explanation:** The distances from (0, 0) to other cells are: [0,1]
// ```
//
// **Example 2:**
//
// ```
// **Input:** rows = 2, cols = 2, rCenter = 0, cCenter = 1
// **Output:** [[0,1],[0,0],[1,1],[1,0]]
// **Explanation:** The distances from (0, 1) to other cells are: [0,1,1,2]
// The answer [[0,1],[1,1],[0,0],[1,0]] would also be accepted as correct.
// ```
//
// **Example 3:**
//
// ```
// **Input:** rows = 2, cols = 3, rCenter = 1, cCenter = 2
// **Output:** [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
// **Explanation:** The distances from (1, 2) to other cells are: [0,1,1,2,2,3]
// There are other answers that would also be accepted as correct, such as [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]].
// ```
//
// **Constraints:**
//
// *   `1 <= rows, cols <= 100`
// *   `0 <= rCenter < rows`
// *   `0 <= cCenter < cols`

pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for r in 0..rows {
        for c in 0..cols {
            result.push(vec![r, c]);
        }
    }

    result.sort_unstable_by_key(|x| (x[0] - r_center).abs() + (x[1] - c_center).abs());
    return result;
}

#[test]
pub fn t1() {
    assert_eq!(
        all_cells_dist_order(1, 2, 0, 0),
        vec![vec![0, 0], vec![0, 1]]
    );
}
