// https://leetcode.com/problems/projection-area-of-3d-shapes
//
// You are given an `n x n` `grid` where we place some `1 x 1 x 1` cubes that are axis-aligned with the `x`, `y`, and `z` axes.
//
// Each value `v = grid[i][j]` represents a tower of `v` cubes placed on top of the cell `(i, j)`.
//
// We view the projection of these cubes onto the `xy`, `yz`, and `zx` planes.
//
// A **projection** is like a shadow, that maps our **3-dimensional** figure to a **2-dimensional** plane. We are viewing the "shadow" when looking at the cubes from the top, the front, and the side.
//
// Return _the total area of all three projections_.
//
// **Example 1:**
//
// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/02/shadow.png)
// ```
// **Input:** grid = [[1,2],[3,4]]
// **Output:** 17
// **Explanation:** Here are the three projections ("shadows") of the shape made with each axis-aligned plane.
// ```
//
// **Example 2:**
//
// ```
// **Input:** grid = [[2]]
// **Output:** 5
// ```
//
// **Example 3:**
//
// ```
// **Input:** grid = [[1,0],[0,2]]
// **Output:** 8
// ```
//
// **Constraints:**
//
// *   `n == grid.length == grid[i].length`
// *   `1 <= n <= 50`
// *   `0 <= grid[i][j] <= 50`

pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let xy = grid
        .iter()
        .map(|row| row.iter().filter(|&&v| v > 0).count() as i32)
        .sum::<i32>();
    let xz = grid
        .iter()
        .map(|row| row.iter().max().unwrap())
        .sum::<i32>();
    let yz = (0..grid.len())
        .map(|i| grid.iter().map(|row| row[i]).max().unwrap_or(0))
        .sum::<i32>();

    return xy + xz + yz;
}

pub fn projection_area_fast(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for i in 0..grid.len() {
        let mut xz = 0;
        let mut yz = 0;
        for j in 0..grid[i].len() {
            if grid[i][j] > 0 {
                res += 1;
            }
            xz = xz.max(grid[i][j]);
            yz = yz.max(grid[j][i]);
        }
        res += xz + yz;
    }

    return res;
}

#[test]
pub fn t1() {
    assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(projection_area_fast(vec![vec![1, 2], vec![3, 4]]), 17);
}
